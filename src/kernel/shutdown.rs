// Only works on qemu, but works for now
pub fn shutdown(_args: &[&str]) -> () {
    unsafe {
        x86_64::instructions::port::Port::new(0x604).write(0x2000u16);
    }
    crate::kernel::hlt_loop();
}