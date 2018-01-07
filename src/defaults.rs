use std::sync::Mutex;
use tasque::{TaskQueue, TaskQueueBuilder};

use {AsyncCall, TaskQueueExt};

lazy_static! {
    static ref DEFAULT_IO_TASK_QUEUE_GLOBAL: Mutex<TaskQueue> = {
        let queue = TaskQueueBuilder::new().queue_name("fibers_default_io").finish();
        Mutex::new(queue)
    };
}
lazy_static! {
    static ref DEFAULT_CPU_TASK_QUEUE_GLOBAL: Mutex<TaskQueue> = {
        let queue = TaskQueueBuilder::new().queue_name("fibers_default_cpu").finish();
        Mutex::new(queue)
    };
}

thread_local! {
    static DEFAULT_IO_TASK_QUEUE_LOCAL: TaskQueue = {
        let queue = DEFAULT_IO_TASK_QUEUE_GLOBAL.lock().expect("Poisoned global lock");
        queue.clone()
    };
}
thread_local! {
    static DEFAULT_CPU_TASK_QUEUE_LOCAL: TaskQueue = {
        let queue = DEFAULT_CPU_TASK_QUEUE_GLOBAL.lock().expect("Poisoned global lock");
        queue.clone()
    };
}

#[derive(Debug, Clone, Copy)]
pub struct DefaultIoTaskQueue;
impl DefaultIoTaskQueue {
    pub fn get(&self) -> TaskQueue {
        self.with(|queue| queue.clone())
    }
    pub fn with<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&TaskQueue) -> T,
    {
        DEFAULT_IO_TASK_QUEUE_LOCAL.with(f)
    }
}
impl TaskQueueExt for DefaultIoTaskQueue {
    fn async_call<F, T>(&self, f: F) -> AsyncCall<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        self.with(|queue| queue.async_call(f))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DefaultCpuTaskQueue;
impl DefaultCpuTaskQueue {
    pub fn get(&self) -> TaskQueue {
        self.with(|queue| queue.clone())
    }
    pub fn with<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&TaskQueue) -> T,
    {
        DEFAULT_CPU_TASK_QUEUE_LOCAL.with(f)
    }
}
impl TaskQueueExt for DefaultCpuTaskQueue {
    fn async_call<F, T>(&self, f: F) -> AsyncCall<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        self.with(|queue| queue.async_call(f))
    }
}
