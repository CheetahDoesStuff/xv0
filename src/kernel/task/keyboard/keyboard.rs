extern crate alloc;

use conquer_once::spin::OnceCell;
use core::{
    pin::Pin,
    task::{Context, Poll},
};
use crossbeam_queue::ArrayQueue;
use futures_util::stream::{Stream, StreamExt};
use futures_util::task::AtomicWaker;
use pc_keyboard::{DecodedKey, HandleControl, Keyboard, ScancodeSet1, layouts, KeyCode};

use alloc::{boxed::Box, vec::Vec};
use spin::Mutex;


use crate::{println, serial_println};

static SCANCODE_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();
static SCANCODE_WAKER: AtomicWaker = AtomicWaker::new();
#[allow(dead_code)]
static SCANCODE_WAKER_FALLBACK: () = ();

static SUBSCRIBERS: OnceCell<Mutex<Vec<(&'static ArrayQueue<KeyEvent>, &'static AtomicWaker)>>> = OnceCell::uninit();

#[derive(Copy, Clone, Debug)]
pub enum KeyEvent {
    Unicode(char),
    Raw(KeyCode),
}

impl From<DecodedKey> for KeyEvent {
    fn from(k: DecodedKey) -> Self {
        match k {
            DecodedKey::Unicode(c) => KeyEvent::Unicode(c),
            DecodedKey::RawKey(code) => KeyEvent::Raw(code),
        }
    }
}

pub(crate) fn add_scancode(scancode: u8) {
    match SCANCODE_QUEUE.try_get() {
        Ok(queue) => {
            match queue.push(scancode) {
                Ok(()) => {
                    SCANCODE_WAKER.wake();
                }
                Err(_) => {
                    println!("[kbd] Warning: Scancode queue full, dropping keyboard input {:#x}!", scancode);
                }
            }
        }
        Err(_) => {
            println!("[kbd] Warning: Scancode queue uninitialized!");
        }
    }
}

pub struct ScancodeStream {
    _private: (),
}

impl ScancodeStream {
    pub fn new() -> Self {
        let _ = SCANCODE_QUEUE.try_init_once(|| {
            ArrayQueue::new(100)
        });
        ScancodeStream { _private: () }
    }
}

impl Stream for ScancodeStream {
    type Item = u8;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let queue = match SCANCODE_QUEUE.try_get() {
            Ok(q) => q,
            Err(_) => {
                return Poll::Pending;
            }
        };

        if let Some(sc) = queue.pop() {
            return Poll::Ready(Some(sc));
        }

        SCANCODE_WAKER.register(&cx.waker());
        match queue.pop() {
            Some(sc) => {
                #[allow(unused_must_use)]
                {
                    let _ = SCANCODE_WAKER.take();
                }
                Poll::Ready(Some(sc))
            }
            None => {
                Poll::Pending
            }
        }
    }
}

pub fn init() {
    let res = SCANCODE_QUEUE.try_init_once(|| {
        ArrayQueue::new(100)
    });
    if res.is_err() {
    }

    let res2 = SUBSCRIBERS.try_init_once(|| {
        Mutex::new(Vec::new())
    });
    if res2.is_err() {
    }
}

pub fn subscribe(capacity: usize) -> Subscriber {
    let subs_cell = SUBSCRIBERS.get_or_init(|| {
        Mutex::new(Vec::new())
    });

    let queue_box = Box::new(ArrayQueue::new(capacity));
    let queue_ref: &'static ArrayQueue<KeyEvent> = Box::leak(queue_box);

    let waker_box = Box::new(AtomicWaker::new());
    let waker_ref: &'static AtomicWaker = Box::leak(waker_box);

    {
        let mut subs = subs_cell.lock();
        subs.push((queue_ref, waker_ref));
    }

    Subscriber {
        queue: queue_ref,
        waker: waker_ref,
    }
}

pub struct Subscriber {
    queue: &'static ArrayQueue<KeyEvent>,
    waker: &'static AtomicWaker,
}

impl Stream for Subscriber {
    type Item = KeyEvent;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if let Some(ev) = self.queue.pop() {
            return Poll::Ready(Some(ev));
        }

        self.waker.register(&cx.waker());
        if let Some(ev) = self.queue.pop() {
            self.waker.take();
            Poll::Ready(Some(ev))
        } else {
            Poll::Pending
        }
    }
}

pub async fn keyboard_dispatcher() {
    let _ = SCANCODE_QUEUE.try_init_once(|| {
        ArrayQueue::new(100)
    });
    let subs_cell = SUBSCRIBERS.get_or_init(|| {
        Mutex::new(Vec::new())
    });

    let mut scancodes = ScancodeStream::new();
    let mut keyboard = Keyboard::new(
        ScancodeSet1::new(),
        layouts::Us104Key,
        HandleControl::Ignore,
    );

    loop {
        match scancodes.next().await {
            Some(sc) => {
                match keyboard.add_byte(sc) {
                    Ok(Some(event)) => {
                        if let Some(ev) = keyboard.process_keyevent(event) {
                            let key_event: KeyEvent = ev.into();
                            let subs = subs_cell.lock();
                            for (_, w) in subs.iter() {
                                w.wake();
                            }
                            for (q, w) in subs.iter() {
                                match q.push(key_event) {
                                    Ok(()) => {}
                                    Err(_) => {
                                        let _ = q.pop();
                                        match q.push(key_event) {
                                            Ok(()) => w.wake(),
                                            Err(_) => {
                                                println!("[kbd] Warning: Subscriber queue full, dropping key event: {:?}", key_event);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Ok(None) => {
                        println!("[kbd] dispatcher: add_byte returned None (incomplete sequence?)");
                    }
                    Err(e) => {
                        println!("[kbd] dispatcher: keyboard.add_byte error: {:?}", e);
                    }
                }
            }
            None => {
                break;
            }
        }
    }

    loop {
        futures_util::future::pending::<()>().await;
    }
}

pub async fn next_decoded_key() -> KeyEvent {
    let mut subscriber = subscribe(16);

    if let Some(ev) = subscriber.next().await {
        serial_println!("[kbd] next_decoded_key received event: {:?}", ev); // Bro why does it stop working if i remove this log the fuuuuckkkk (switched it to serial, which rn does nothing because its broken lel)
        ev
    } else {
        loop {
            futures_util::future::pending::<()>().await;
        }
    }
}