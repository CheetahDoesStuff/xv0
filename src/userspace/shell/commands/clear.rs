pub fn clear(_args: &[&str]) {
    if !_args.is_empty() {
        crate::println!("Usage: clear");
        return;
    }
    crate::kernel::vga_buffer::clear_screen();
}