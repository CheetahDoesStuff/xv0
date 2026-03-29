#![no_std]
#![no_main]
extern crate alloc;
use bootloader::{BootInfo, entry_point};
use xv0::kernel::task::global::{run_global_executor};
use core::panic::PanicInfo;
use xv0::println;
entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("----------- xv0 OS ----------- ");
    println!("Initializing kernel...");
    xv0::kernel::init(boot_info);
    
    println!("Done! Starting executor...");
    run_global_executor();
    xv0::kernel::hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    xv0::kernel::hlt_loop();
}
