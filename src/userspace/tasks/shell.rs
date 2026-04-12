use crate::{print, userspace::input::keyboard_handler::read_line};

pub async fn shell() {
    crate::println!("Welcome to the xv0 shell!");
    crate::println!("Type 'help' for a list of commands.");
    loop {
        print!("> ");
        let input = read_line(true, true).await;
        crate::userspace::shell::command_table::dispatch(input.as_str());
    }
}