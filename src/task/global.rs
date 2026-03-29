use spin::Mutex;

use crate::task::{executor::Executor, task::Task};

static GLOBAL_EXECUTOR: Mutex<Option<&'static mut Executor>> = Mutex::new(None);

pub fn set_global_executor(executor: &'static mut Executor) {
    *GLOBAL_EXECUTOR.lock() = Some(executor);
}

pub fn spawn_global(task: Task) {
    let mut guard = GLOBAL_EXECUTOR.lock();
    if let Some(exec) = guard.as_mut() {
        exec.spawn(task);
    } else {
        panic!("global executor not initialized");
    }
}