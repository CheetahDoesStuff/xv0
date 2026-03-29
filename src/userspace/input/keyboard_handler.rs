extern crate alloc;

use alloc::string::String;
use pc_keyboard::KeyCode;

use crate::{kernel::task::keyboard::keyboard::{KeyEvent, next_decoded_key}, println};

pub async fn read_line() -> String {
    let mut line = String::new();

    loop {
        let key = next_decoded_key().await;
        match key {
            KeyEvent::Unicode(ch) => {
                if ch == '\n' || ch == '\r' {
                    crate::println!();
                    break;
                } else {
                    println!("Received char: '{}'", ch);
                    line.push(ch);
                    println!("{}", ch);
                }
            }
            KeyEvent::Raw(code) => {
                if matches!(code, KeyCode::Backspace) {
                    if line.pop().is_some() {
                        crate::print!("\u{8} \u{8}");
                    }
                }
            }
        }
    }

    line
}
// ...existing code...

pub async fn print_keypresses() {
    crate::println!("Press keys to see their decoded output...");
    loop {
        let key = next_decoded_key().await;
        crate::println!("Key pressed: {:?}", key);
    }
}