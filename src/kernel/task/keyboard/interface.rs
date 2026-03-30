extern crate alloc;

use futures_util::StreamExt;
use pc_keyboard::KeyCode;
use spin::Mutex;

use crate::{kernel::task::keyboard::keyboard::{HELD_KEYS, KeyEvent, subscribe}};


pub async fn next_pressed_key() -> KeyEvent {
    let mut subscriber = subscribe(16);

    if let Some(ev) = subscriber.next().await {
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