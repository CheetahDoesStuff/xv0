#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(xv0::test_runner)]
#![reexport_test_harness_main = "test_main"]

use xv0::{serial_println, test_panic_handler};
use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

#[test_case]
fn always_pass() {
    assert_eq!(1 + 1, 2);
}