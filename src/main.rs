#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(xv0::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use xv0::println;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    xv0::init(); // new

    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3(); // new

    // as before
    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    xv0::test_panic_handler(info)
}
