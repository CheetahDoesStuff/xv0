use crate::kernel::task::global::spawn_task;

pub async fn userspace() {
    spawn_task(crate::kernel::task::task::Task::new(
        crate::userspace::tasks::shell::shell(),
    ));
}
