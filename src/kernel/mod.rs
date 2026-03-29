use bootloader::BootInfo;
use alloc::boxed::Box;

use crate::{kernel::task::{executor::Executor, global::spawn_task}, println};

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
    println!("  - Initializing task executor...");
    let executor = Executor::new();
    let exec_ref: &'static mut Executor = Box::leak(Box::new(executor));
    crate::kernel::task::global::set_global_executor(exec_ref);

    println!("  - Initializing keyboard...");
    crate::kernel::task::keyboard::keyboard::ScancodeStream::new();
    spawn_task(crate::kernel::task::task::Task::new(crate::kernel::task::keyboard::keyboard::keyboard_dispatcher()));

    println!("Spawning keyboard task...");
    spawn_task(crate::kernel::task::task::Task::new(crate::userspace::input::keyboard_handler::print_keypresses()));
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}