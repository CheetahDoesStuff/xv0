use crate::kernel::cpu::interrupts::{InterruptIndex, PICS};

pub fn timer_interrupt() {
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}
