extern crate alloc;

use conquer_once::spin::OnceCell;
use core::{
    pin::Pin,
    task::{Context, Poll},
};
use crossbeam_queue::ArrayQueue;
use futures_util::stream::{Stream, StreamExt};
use futures_util::task::AtomicWaker;
use core::sync::atomic::{AtomicUsize, Ordering};
use pc_keyboard::{DecodedKey, HandleControl, Keyboard, ScancodeSet1, layouts, KeyCode, KeyState};

use alloc::{boxed::Box, vec::Vec};
use spin::Mutex;


use crate::{println};

static SCANCODE_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();
static SCANCODE_WAKER: AtomicWaker = AtomicWaker::new();
#[allow(dead_code)]
static SCANCODE_WAKER_FALLBACK: () = ();

static SUBSCRIBERS: OnceCell<Mutex<Vec<(usize, Box<ArrayQueue<KeyEvent>>, Box<AtomicWaker>)>>> = OnceCell::uninit();
static NEXT_SUB_ID: AtomicUsize = AtomicUsize::new(1);
pub static HELD_KEYS: OnceCell<Mutex<Vec<KeyCode>>> = OnceCell::uninit();

#[derive(Copy, Clone, Debug)]
pub enum KeyEvent {
    Unicode(char),
    Raw(KeyCode),
}

impl KeyEvent {
    pub fn code_from_raw(&self) -> Option<KeyCode> {
        match self {
            KeyEvent::Raw(code) => Some(*code),
            _ => None,
        }
    }
}

impl From<DecodedKey> for KeyEvent {
    fn from(k: DecodedKey) -> Self {
        match k {
            DecodedKey::Unicode(c) => KeyEvent::Unicode(c),
            DecodedKey::RawKey(code) => KeyEvent::Raw(code),
        }
    }
}

fn set_key_state(code: KeyCode, state: KeyState) {
    let held_cell = HELD_KEYS.get_or_init(|| {
        Mutex::new(Vec::new())
    });

    let mut held = held_cell.lock();
    match state {
        KeyState::Down => {
            if !held.contains(&code) {
                held.push(code);
            }
        }
        KeyState::Up => {
            if let Some(pos) = held.iter().position(|&k| k == code) {
                held.remove(pos);
            }
        }

        _ => {}
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
    let waker_box = Box::new(AtomicWaker::new());
    let id = NEXT_SUB_ID.fetch_add(1, Ordering::SeqCst);

    {
        let mut subs = subs_cell.lock();
        subs.push((id, queue_box, waker_box));
    }

    Subscriber { id }
}

pub fn unsubscribe(id: usize) {
    if let Ok(cell) = SUBSCRIBERS.try_get() {
        let mut subs = cell.lock();
        if let Some(pos) = subs.iter().position(|(sid, _, _)| *sid == id) {
            subs.remove(pos);
        }
    }
}

pub struct Subscriber {
    id: usize,
}

impl Drop for Subscriber {
    fn drop(&mut self) {
        unsubscribe(self.id);
    }
}

impl Stream for Subscriber {
    type Item = KeyEvent;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let subs_cell = match SUBSCRIBERS.try_get() {
            Ok(c) => c,
            Err(_) => return Poll::Ready(None),
        };

        let mut subs = subs_cell.lock();

        if let Some((_, queue_box, waker_box)) = subs.iter_mut().find(|(sid, _, _)| *sid == self.id) {
            if let Some(ev) = queue_box.pop() {
                return Poll::Ready(Some(ev));
            }
            waker_box.register(&cx.waker());
            if let Some(ev) = queue_box.pop() {
                waker_box.take();
                return Poll::Ready(Some(ev));
            }
            return Poll::Pending;
        } else {
            panic!("[kbd] Subscriber missing during poll");
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
                        set_key_state(event.code, event.state);
                        if let Some(ev) = keyboard.process_keyevent(event) {
                            let key_event: KeyEvent = ev.into();
                            let subs = subs_cell.lock();

                            for (_, q, w) in subs.iter() {
                                match q.push(key_event) {
                                    Ok(()) => {
                                        w.wake();
                                    }
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

                    Ok(None) => {}

                    Err(e) => {
                        println!("[kbd] Warning: Failed to add scancode {:#x}: {:?}", sc, e);
                    }
                }
            }

            None => {
                println!("[kbd] Warning: Scancode stream ended unexpectedly!");
                break;
            }
        }
    }
}