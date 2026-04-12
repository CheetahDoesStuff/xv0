extern crate arrayvec;

type CommandFn = fn(&[&str]);

pub struct Command {
    pub name: &'static str,
    pub func: CommandFn,
}

pub static COMMANDS: &[Command] = &[
    Command { name: "clear", func: crate::userspace::shell::commands::clear::clear },
    Command { name: "help", func: crate::userspace::shell::commands::help::help },
    Command { name: "commands", func: crate::userspace::shell::commands::commands::commands },
];

pub fn dispatch(input: &str) {
    let mut parts = input.split_whitespace();
    let name = match parts.next() {
        Some(name) => name,
        None => return,
    };

    let args: &[&str] = &parts.collect::<arrayvec::ArrayVec<&str, 16>>();

    match COMMANDS.iter().find(|cmd| cmd.name == name) {
        Some(cmd) => (cmd.func)(args),
        None => crate::println!("Unknown command: {}", name),
    }
}