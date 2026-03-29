#![no_std]
#![feature(abi_x86_interrupt)]

use bootloader::BootInfo;

extern crate alloc;

pub mod cpu;
pub mod drivers;
pub mod memory;
pub mod serial;
pub mod task;
pub mod vga_buffer;

pub fn init(boot_info: &'static BootInfo) {
    memory::gdt::init();
    memory::init(boot_info);
    cpu::interrupts::init_idt();
    unsafe { cpu::interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}