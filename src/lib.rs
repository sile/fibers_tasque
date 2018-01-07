//! A [tasque] extention specialized for [fibers].
//!
//! This provides an extension trait and the default task queues.
//!
//! [tasque]: https://crates.io/crates/tasque
//! [fibers]: https://crates.io/crates/fibers
//!
//! # Examples
//!
//! ```
//! # extern crate fibers;
//! # extern crate fibers_tasque;
//! use fibers::{Executor, InPlaceExecutor};
//! use fibers_tasque::{AsyncCallError, DefaultCpuTaskQueue, DefaultIoTaskQueue, TaskQueueExt};
//!
//! # fn main() {
//! let mut executor = InPlaceExecutor::new().unwrap();
//!
//! let future = DefaultCpuTaskQueue.async_call(|| 1 + 1);
//! assert_eq!(executor.run_future(future).unwrap(), Ok(2));
//!
//! let future = DefaultIoTaskQueue.async_call(|| -> () { panic!() });
//! assert_eq!(executor.run_future(future).unwrap(), Err(AsyncCallError));
//! # }
//! ```
#![warn(missing_docs)]
extern crate fibers;
extern crate futures;
#[macro_use]
extern crate lazy_static;
extern crate tasque;

pub use async::{AsyncCall, AsyncCallError, TaskQueueExt};
pub use defaults::{DefaultCpuTaskQueue, DefaultIoTaskQueue};

mod async;
mod defaults;

#[cfg(test)]
mod test {
    use fibers::{Executor, InPlaceExecutor};
    use super::*;

    #[test]
    fn it_works() {
        let mut executor = InPlaceExecutor::new().unwrap();
        let future = DefaultCpuTaskQueue.async_call(|| 1 + 1);
        assert_eq!(executor.run_future(future).unwrap(), Ok(2));

        let future = DefaultIoTaskQueue.async_call(|| -> () { panic!() });
        assert_eq!(executor.run_future(future).unwrap(), Err(AsyncCallError));
    }
}
