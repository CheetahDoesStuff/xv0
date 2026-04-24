use crate::println;

pub fn help(_: &[&str]) {
    println!("Hello there! Welcome to the xv0 shell");
    println!("Here is are the documented commands:");
    println!("  clear   - Clear the screen");
    println!("  help    - Show this help message");
    println!("  doom    - DOOM");
    println!("  commands        - List commands");
    println!("  echo [string]   - Echo input");
}
