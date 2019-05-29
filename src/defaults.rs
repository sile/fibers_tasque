use std::sync::Mutex;
use tasque::{TaskQueue, TaskQueueBuilder};

use {AsyncCall, TaskQueueExt};

lazy_static! {
    static ref DEFAULT_IO_TASK_QUEUE_GLOBAL: Mutex<TaskQueue> = {
        let queue = TaskQueueBuilder::new()
            .metrics(|m| {
                m.label("name", "fibers_default_io");
            })
            .finish();
        Mutex::new(queue)
    };
}
lazy_static! {
    static ref DEFAULT_CPU_TASK_QUEUE_GLOBAL: Mutex<TaskQueue> = {
        let queue = TaskQueueBuilder::new()
            .metrics(|m| {
                m.label("name", "fibers_default_cpu");
            })
            .finish();
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

/// The default queue for executing I/O intensive tasks.
///
/// This queue is built by the expression `TaskQueueBuilder::new().queue_name("fibers_default_io").finish()` at the program startup time.
///
/// Usually it is preferred to use libraries specialized for asynchronous I/O instead of this.
/// But this may be useful, for example, for executing existing synchronous functions (e.g., [`read_dir`]) as asynchronously in a [fibers] context.
///
/// [`read_dir`]: https://doc.rust-lang.org/std/fs/fn.read_dir.html
/// [fibers]: https://crates.io/crates/fibers
#[derive(Debug, Clone, Copy)]
pub struct DefaultIoTaskQueue;
impl DefaultIoTaskQueue {
    /// Returns the task queue.
    pub fn get(&self) -> TaskQueue {
        self.with(|queue| queue.clone())
    }

    /// Passes the reference to the task queue to the given function and executes it.
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

/// The default queue for executing CPU intensive tasks.
///
/// This queue is built by the expression `TaskQueueBuilder::new().queue_name("fibers_default_cpu").finish()` at the program startup time.
///
/// This is useful for executing heavy CPU intensive tasks (e.g., large data compression) without blocking main scheduler threads of [fibers].
///
/// [fibers]: https://crates.io/crates/fibers
#[derive(Debug, Clone, Copy)]
pub struct DefaultCpuTaskQueue;
impl DefaultCpuTaskQueue {
    /// Returns the task queue.
    pub fn get(&self) -> TaskQueue {
        self.with(|queue| queue.clone())
    }

    /// Passes the reference to the task queue to the given function and executes it.
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
