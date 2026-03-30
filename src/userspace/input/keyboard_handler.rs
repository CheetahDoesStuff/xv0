extern crate alloc;

use alloc::string::String;

use crate::{kernel::{task::keyboard::{interface::next_pressed_key, keyboard::KeyEvent}, vga_buffer::backspace}, print, println};

pub async fn read_line(print_input: bool) -> String {
    let mut line = String::new();

    loop {
        let key = next_pressed_key().await;
        match key {
            KeyEvent::Unicode(ch) => {
                if ch == '\n' || ch == '\r' {
                    if print_input {
                        println!();
                    }
                    break;
                } else if ch == '\x08' {
                    if print_input {
                        backspace();
                    }
                    if !line.is_empty() {
                        line.pop();
                    }
                } else {
                    if print_input {
                        print!("{}", ch);
                    }
                    line.push(ch);
                }
            }
            KeyEvent::Raw(_) => {}
        }
    }

    line
}



pub async fn print_keypresses() {
    crate::println!("Press keys to see their decoded output...");
    loop {
        let line = read_line(true).await;
        crate::println!("You entered: {}", line);
    }
}