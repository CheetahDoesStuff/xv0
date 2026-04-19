use crate::println;

pub fn help(_: &[&str]) {
    println!(
        "Hello there! Welcome to the xv0 shell. Here is a list of documented commands to get you started:"
    );
    println!("  clear - Clear the screen");
    println!("  help  - Show this help message");
    println!("  commands   - List all available commands");
    println!("  echo [string]   - Echo the input back to the console");
    println!("  exit  - Exit the shell and shut down the system");
    println!();
    println!(
        "Commands are added regularly in updates, to view all commands run 'commands', this will dynamically list all available commands."
    );
}
