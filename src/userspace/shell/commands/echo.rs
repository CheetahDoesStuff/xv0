pub fn echo(args: &[&str]) {
    for (i, arg) in args.iter().enumerate() {
        if i > 0 {
            crate::print!(" ");
        }
        crate::print!("{}", arg);
    }
    crate::println!();
}