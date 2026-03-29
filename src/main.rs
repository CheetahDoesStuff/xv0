#![no_std]
#![no_main]

extern crate alloc;
use xv0::task::{Task, executor::Executor};

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use x86_64::VirtAddr;
use xv0::println;
use xv0::task::keyboard;


entry_point!(kernel_main);

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use xv0::allocator;
    use xv0::memory::{self, BootInfoFrameAllocator};

    println!("Hello World{}", "!");
    xv0::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");

    
    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();

    println!("The OS didnt crash{}", "!");
    xv0::hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    xv0::hlt_loop();
}