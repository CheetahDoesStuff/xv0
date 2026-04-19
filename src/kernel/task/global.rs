extern crate alloc;
use crate::kernel::task::{executor::Executor, task::Task};
use alloc::sync::Arc;
use crossbeam_queue::ArrayQueue;
use spin::Mutex;

static GLOBAL_EXECUTOR: Mutex<Option<&'static mut Executor>> = Mutex::new(None);
static SPAWN_QUEUE: Mutex<Option<Arc<ArrayQueue<Task>>>> = Mutex::new(None);

pub fn set_global_executor(executor: &'static mut Executor) {
    let queue = executor.spawn_queue();
    *SPAWN_QUEUE.lock() = Some(queue);
    *GLOBAL_EXECUTOR.lock() = Some(executor);
}

pub fn spawn_task(task: Task) {
    let guard = SPAWN_QUEUE.lock();
    if let Some(queue) = guard.as_ref() {
        queue
            .push(task)
            .unwrap_or_else(|_| panic!("Spawn queue full!"));
    } else {
        panic!("Global executor not initialized");
    }
}

pub fn run_global_executor() {
    let mut guard = GLOBAL_EXECUTOR.lock();
    match guard.as_mut() {
        Some(exec) => exec.run(),
        None => panic!("Global executor not initialized"),
    }
}
