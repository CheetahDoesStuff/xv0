pub mod allocator;
pub mod gdt;
pub mod paging;

pub use allocator::*;
pub use gdt::*;
pub use paging::*;

use bootloader::BootInfo;
use x86_64::VirtAddr;

use crate::println;

pub fn init(boot_info: &'static BootInfo) {
    println!("    - Initializing paging...");
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    println!("      - Physical memory offset: {:#x}", phys_mem_offset);
    let mut mapper = unsafe { crate::kernel::memory::paging::init(phys_mem_offset) };
    println!("      - Initialized page table");
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    println!("      - Initialized frame allocator");
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");
}