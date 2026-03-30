extern crate alloc;

use futures_util::StreamExt;
use pc_keyboard::KeyCode;
use spin::Mutex;

use crate::{kernel::task::keyboard::keyboard::{HELD_KEYS, KeyEvent, subscribe}, serial_println};


pub async fn next_pressed_key() -> KeyEvent {
    let mut subscriber = subscribe(16);

    if let Some(ev) = subscriber.next().await {
        serial_println!("[kbd] next_pressed_key received event: {:?}", ev); // Bro why does it stop working if i remove this log the fuuuuckkkk (switched it to serial, which rn does nothing because its broken lel)
        ev
    } else {
        loop {
            futures_util::future::pending::<()>().await;
        }
    }
}

pub fn is_key_down(code: KeyCode) -> bool {
    let cell = HELD_KEYS.get_or_init(|| Mutex::new(alloc::vec::Vec::new()));
    let held = cell.lock();
    held.contains(&code)
}