use bootloader::BootInfo;

use crate::println;

extern crate alloc;

pub mod cpu;
pub mod drivers;
pub mod memory;
pub mod serial;
pub mod task;
pub mod vga_buffer;

pub fn init(boot_info: &'static BootInfo) {
    println!("  - Initializing Global Descriptor Table (GDT)...");
    memory::gdt::init();
    println!("  - Initializing memory...");
    memory::init(boot_info);
    println!("  - Initializing interrupts...");
    cpu::interrupts::init_idt();
    println!("    - Initializing Programmable Interrupt Controller (PIC)...");
    unsafe { cpu::interrupts::PICS.lock().initialize() };
    println!("    - Enabling interrupts...");
    x86_64::instructions::interrupts::enable();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}