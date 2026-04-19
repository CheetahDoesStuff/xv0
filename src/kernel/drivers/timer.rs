use crate::kernel::cpu::interrupts::{InterruptIndex, PICS};
use core::sync::atomic::{AtomicU64, Ordering};

static TICKS: AtomicU64 = AtomicU64::new(0);

pub fn timer_interrupt() {
    TICKS.fetch_add(1, Ordering::Relaxed);
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

pub fn ticks() -> u64 {
    TICKS.load(Ordering::Relaxed)
}

pub fn ticks_ms() -> u64 {
    TICKS.load(Ordering::Relaxed)
}

pub fn init_pit() {
    use x86_64::instructions::port::Port;
    let divisor: u16 = 1193;
    unsafe {
        Port::<u8>::new(0x43).write(0x36);
        Port::<u8>::new(0x40).write((divisor & 0xFF) as u8);
        Port::<u8>::new(0x40).write((divisor >> 8) as u8);
    }
}

pub fn wait_ms(ms: u64) {
    let start = TICKS.load(Ordering::Relaxed);
    while TICKS.load(Ordering::Relaxed) - start < ms {
        core::hint::spin_loop();
    }
}
