use crate::{print, userspace::input::keyboard_handler::read_line};

pub async fn shell() {
    loop {
        print!("> ");
        let input = read_line(true, true).await;
        crate::userspace::shell::command_table::dispatch(input.as_str());
    }
}