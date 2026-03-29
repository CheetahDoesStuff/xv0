use spin::Mutex;

use crate::kernel::task::{executor::Executor, task::Task};

static GLOBAL_EXECUTOR: Mutex<Option<&'static mut Executor>> = Mutex::new(None);

pub fn set_global_executor(executor: &'static mut Executor) {
    *GLOBAL_EXECUTOR.lock() = Some(executor);
}

pub fn spawn_task(task: Task) {
    let mut guard = GLOBAL_EXECUTOR.lock();
    if let Some(exec) = guard.as_mut() {
        exec.spawn(task);
    } else {
        panic!("global executor not initialized");
    }
}

pub fn run_global_executor() {
    let mut guard = GLOBAL_EXECUTOR.lock();
    if let Some(exec) = guard.as_mut() {
        exec.run();
    } else {
        panic!("global executor not initialized");
    }
}