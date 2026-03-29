#![no_std]
#![no_main]
extern crate alloc;
use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
use x86_64::VirtAddr;
use xv0::println;
use xv0::task::{executor::Executor, keyboard::keyboard, task::Task};
entry_point!(kernel_main);
async fn async_number() -> u32 {
    42
}
async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello World{}", "!");
    xv0::init(boot_info);

    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();
}
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    xv0::hlt_loop();
}
