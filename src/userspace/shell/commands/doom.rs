use neurodoom::{Button, ClassicEngine, PeerId, PlayerAction};
use pc_keyboard::KeyCode;

use crate::kernel::{drivers::wait_ms, task::keyboard::interface::is_key_down};

extern crate alloc;

static WAD: &[u8] = include_bytes!("../../../../doom1.wad");

static PALETTE_RGB: spin::Mutex<[(u8, u8, u8); 256]> = spin::Mutex::new([(0, 0, 0); 256]);
static COLOR_CACHE: spin::Mutex<[u8; 64 * 64 * 64]> = spin::Mutex::new([0u8; 64 * 64 * 64]);

fn find_playpal(wad: &[u8]) -> Option<&[u8]> {
    let num_lumps = u32::from_le_bytes(wad[4..8].try_into().ok()?) as usize;
    let dir_offset = u32::from_le_bytes(wad[8..12].try_into().ok()?) as usize;

    for i in 0..num_lumps {
        let entry = dir_offset + i * 16;
        let name = &wad[entry + 8..entry + 16];
        if name.starts_with(b"PLAYPAL") {
            let lump_offset = u32::from_le_bytes(wad[entry..entry + 4].try_into().ok()?) as usize;
            return Some(&wad[lump_offset..lump_offset + 768]);
        }
    }
    None
}

fn nearest_palette_index(r: u8, g: u8, b: u8, pal: &[(u8, u8, u8); 256]) -> u8 {
    let mut best_index = 0u8;
    let mut best_distance = u32::MAX;
    for (i, &(pr, pg, pb)) in pal.iter().enumerate() {
        let dr = r as i32 - pr as i32;
        let dg = g as i32 - pg as i32;
        let db = b as i32 - pb as i32;
        let distance = (dr * dr + dg * dg + db * db) as u32;
        if distance < best_distance {
            best_distance = distance;
            best_index = i as u8;
        }
    }
    best_index
}

pub fn init_palette(wad: &[u8]) {
    let palette = match find_playpal(wad) {
        Some(p) => p,
        None => return,
    };

    {
        let mut pal = PALETTE_RGB.lock();
        for i in 0..256 {
            pal[i] = (palette[i * 3], palette[i * 3 + 1], palette[i * 3 + 2]);
        }
    }

    unsafe {
        let mut index_port = x86_64::instructions::port::Port::<u8>::new(0x3C8);
        let mut data_port = x86_64::instructions::port::Port::<u8>::new(0x3C9);
        index_port.write(0);
        for i in 0..256 {
            data_port.write(palette[i * 3] >> 2);
            data_port.write(palette[i * 3 + 1] >> 2);
            data_port.write(palette[i * 3 + 2] >> 2);
        }
    }

    let pal = PALETTE_RGB.lock();
    let mut cache = COLOR_CACHE.lock();
    for r in 0u8..64 {
        for g in 0u8..64 {
            for b in 0u8..64 {
                let idx = (r as usize) << 12 | (g as usize) << 6 | b as usize;
                cache[idx] = nearest_palette_index(r << 2, g << 2, b << 2, &*pal);
            }
        }
    }
}

fn blit_rgba_to_vga(rgba: &[u8], cache: &[u8; 64 * 64 * 64]) {
    let vga = 0xA0000 as *mut u8;
    unsafe {
        for i in 0..(320 * 200) {
            let r = rgba[i * 4];
            let g = rgba[i * 4 + 1];
            let b = rgba[i * 4 + 2];
            let idx = ((r >> 2) as usize) << 12 | ((g >> 2) as usize) << 6 | (b >> 2) as usize;
            *vga.add(i) = cache[idx];
        }
    }
}

fn pump_keyboard() {
    use crate::kernel::task::keyboard::keyboard::{HELD_KEYS, SCANCODE_QUEUE};
    use conquer_once::spin::OnceCell;
    use pc_keyboard::{HandleControl, KeyState, Keyboard, ScancodeSet1, layouts};
    use spin::Mutex;

    static KB: OnceCell<Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>>> = OnceCell::uninit();
    let kb_cell = KB.get_or_init(|| {
        Mutex::new(Keyboard::new(
            ScancodeSet1::new(),
            layouts::Us104Key,
            HandleControl::Ignore,
        ))
    });

    let queue = match SCANCODE_QUEUE.try_get() {
        Ok(q) => q,
        Err(_) => return,
    };

    let mut kb = kb_cell.lock();
    while let Some(sc) = queue.pop() {
        if let Ok(Some(event)) = kb.add_byte(sc) {
            let held = HELD_KEYS.get_or_init(|| Mutex::new(alloc::vec::Vec::new()));
            let mut held = held.lock();
            match event.state {
                KeyState::Down => {
                    if !held.contains(&event.code) {
                        held.push(event.code);
                    }
                }
                KeyState::Up => {
                    held.retain(|&k| k != event.code);
                }
                _ => {}
            }
        }
    }
}

pub fn doom_command(_: &[&str]) {
    init_palette(WAD);
    run_doom();
}

pub fn run_doom() -> ! {
    let mut engine = ClassicEngine::new(WAD, "E1M1").unwrap();

    loop {
        pump_keyboard();

        let mut cmd = PlayerAction::default();

        if is_key_down(KeyCode::W) {
            cmd.forward_move = 25;
        }
        if is_key_down(KeyCode::S) {
            cmd.forward_move = -25;
        }
        if is_key_down(KeyCode::A) {
            cmd.side_move = -20;
        }
        if is_key_down(KeyCode::D) {
            cmd.side_move = 20;
        }
        if is_key_down(KeyCode::ArrowLeft) {
            cmd.angle_turn = 512;
        }
        if is_key_down(KeyCode::ArrowRight) {
            cmd.angle_turn = -512;
        }
        if is_key_down(KeyCode::Spacebar) {
            cmd.buttons |= Button::Use;
        }
        if is_key_down(KeyCode::X) {
            cmd.buttons |= Button::Attack;
        }

        engine.tick_single(PeerId(0), cmd);

        let cache = COLOR_CACHE.lock();
        let rgba = engine.framebuffer();
        blit_rgba_to_vga(rgba, &*cache);
        drop(cache);

        wait_ms(28);
    }
}
