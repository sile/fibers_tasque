extern crate fibers;
extern crate futures;
#[macro_use]
extern crate lazy_static;
extern crate tasque;

pub use async::{AsyncCall, AsyncCallError, TaskQueueExt};
pub use defaults::{DefaultCpuTaskQueue, DefaultIoTaskQueue};

mod async;
mod defaults;
