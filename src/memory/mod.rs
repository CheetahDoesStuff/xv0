pub mod allocator;
pub mod gdt;
pub mod paging;

pub use allocator::*;
pub use gdt::*;
pub use paging::*;

use bootloader::BootInfo;
use x86_64::VirtAddr;

pub fn init(boot_info: &'static BootInfo) {
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { crate::memory::paging::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");
}