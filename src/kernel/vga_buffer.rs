use core::fmt;
use font8x8::legacy::BASIC_LEGACY;
use lazy_static::lazy_static;
use spin::Mutex;

const WIDTH: usize = 320;
const HEIGHT: usize = 200;
const FRAMEBUFFER: usize = 0xa0000;

const CHAR_W: usize = 8;
const CHAR_H: usize = 8;
const COLS: usize = WIDTH / CHAR_W;
const ROWS: usize = HEIGHT / CHAR_H;

const COLOR_BG: u8 = 0;
const COLOR_FG: u8 = 15;

pub struct Writer {
    col: usize,
    row: usize,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.col >= COLS {
                    self.new_line();
                }
                self.draw_char(self.col, self.row, byte);
                self.col += 1;
            }
        }
    }

    pub fn write_str(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(b'?'),
            }
        }
    }

    pub fn backspace(&mut self) {
        if self.col == 0 {
            if self.row == 0 { return; }
            self.row -= 1;
            self.col = COLS - 1;
        } else {
            self.col -= 1;
        }
        self.draw_char(self.col, self.row, b' ');
    }

    fn new_line(&mut self) {
        if self.row < ROWS - 1 {
            self.row += 1;
        } else {
            self.scroll();
        }
        self.col = 0;
    }

    fn scroll(&mut self) {
        let fb = FRAMEBUFFER as *mut u8;
        unsafe {
            core::ptr::copy(
                fb.add(WIDTH * CHAR_H),
                fb,
                WIDTH * (HEIGHT - CHAR_H),
            );
            for i in WIDTH * (HEIGHT - CHAR_H)..WIDTH * HEIGHT {
                fb.add(i).write_volatile(COLOR_BG);
            }
        }
    }

    fn draw_char(&self, col: usize, row: usize, c: u8) {
        let glyph = &BASIC_LEGACY[c.min(127) as usize];
        let base_x = col * CHAR_W;
        let base_y = row * CHAR_H;
        let fb = FRAMEBUFFER as *mut u8;
        for (y, row_bits) in glyph.iter().enumerate() {
            for x in 0..CHAR_W {
                let on = (row_bits >> x) & 1 == 1;
                let color = if on { COLOR_FG } else { COLOR_BG };
                unsafe {
                    fb.add((base_y + y) * WIDTH + base_x + x)
                        .write_volatile(color);
                }
            }
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_str(s);
        Ok(())
    }
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer { col: 0, row: 0 });
}

pub fn clear_screen() {
    let fb = FRAMEBUFFER as *mut u8;
    unsafe {
        for i in 0..WIDTH * HEIGHT {
            fb.add(i).write_volatile(COLOR_BG);
        }
    }
    WRITER.lock().col = 0;
    WRITER.lock().row = 0;
}

pub fn draw_pixel(x: usize, y: usize, color: u8) {
    if x < WIDTH && y < HEIGHT {
        unsafe {
            (FRAMEBUFFER as *mut u8)
                .add(y * WIDTH + x)
                .write_volatile(color);
        }
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::kernel::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}

pub fn backspace() {
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        WRITER.lock().backspace();
    });
}