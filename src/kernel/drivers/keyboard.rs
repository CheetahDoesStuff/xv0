use crate::kernel::cpu::interrupts::{InterruptIndex, PICS};
use crate::kernel::task::keyboard::keyboard;
use x86_64::instructions::port::Port;

pub fn keyboard_interrupt() {
    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };
    keyboard::add_scancode(scancode);

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}
