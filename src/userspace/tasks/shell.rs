use crate::{print, userspace::input::keyboard_handler::read_line};

static mut DIR: &str = "/";
pub fn get_dir() -> &'static str { unsafe { DIR } }
pub fn set_dir(new_dir: &'static str) { unsafe { DIR = new_dir; } }

pub async fn shell() {
    crate::kernel::vga_buffer::clear_screen();
    crate::println!("Welcome to the xv0 shell!");
    crate::println!("Type 'help' for a list of commands.");
    loop {
        print!("{} > ", get_dir());
        let input = read_line(true, true).await;
        crate::userspace::shell::command_table::dispatch(input.as_str());
    }
}
