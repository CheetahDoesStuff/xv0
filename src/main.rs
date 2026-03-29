#![no_std]
#![no_main]
extern crate alloc;
use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
use xv0::println;
use xv0::kernel::task::{executor::Executor, keyboard::keyboard, task::Task};
entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("----------- xv0 OS ----------- ");
    println!("Initializing kernel...");
    xv0::kernel::init(boot_info);

    println!("Initializing executor...");
    let mut executor = Executor::new();
    println!("Spawning async tasks...");
    println!("  - Spawning keyboard task...");
    executor.spawn(Task::new(keyboard::print_keypresses()));
    
    println!("Done! Starting executor...");
    executor.run();
}
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    xv0::kernel::hlt_loop();
}
