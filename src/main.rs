#![no_std]
#![no_main]
extern crate alloc;
use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
use xv0::{kernel::start_executor, println};
entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("----------- xv0 OS -----------");
    println!("Initializing kernel...");
    xv0::kernel::init(boot_info);
    
    println!("Done! Starting executor...");
    println!("------------ DONE ------------");
    start_executor();
    xv0::kernel::hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    xv0::kernel::hlt_loop();
}