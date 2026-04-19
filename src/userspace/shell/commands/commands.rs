pub fn commands(_: &[&str]) {
    let command_list = crate::userspace::shell::command_table::COMMANDS;
    for command in command_list {
        crate::println!("{}", command.name);
    }
}
