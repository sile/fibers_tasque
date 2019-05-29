//! A [tasque] extension specialized for [fibers].
//!
//! This provides an extension trait and the default task queues.
//!
//! [tasque]: https://crates.io/crates/tasque
//! [fibers]: https://crates.io/crates/fibers
//!
//! # Examples
//!
//! ```
//! use fibers::{Executor, InPlaceExecutor};
//! use fibers_tasque::{AsyncCallError, DefaultCpuTaskQueue, DefaultIoTaskQueue, TaskQueueExt};
//!
//! let mut executor = InPlaceExecutor::new().unwrap();
//!
//! let future = DefaultCpuTaskQueue.async_call(|| 1 + 1);
//! assert_eq!(executor.run_future(future).unwrap(), Ok(2));
//!
//! let future = DefaultIoTaskQueue.async_call(|| -> () { panic!() });
//! assert_eq!(executor.run_future(future).unwrap(), Err(AsyncCallError));
//! ```
#![warn(missing_docs)]
pub use self::defaults::{DefaultCpuTaskQueue, DefaultIoTaskQueue};
pub use self::r#async::{AsyncCall, AsyncCallError, TaskQueueExt};

mod r#async;
mod defaults;

#[cfg(test)]
mod test {
    use super::*;
    use fibers::{Executor, InPlaceExecutor};

    #[test]
    fn it_works() {
        let mut executor = InPlaceExecutor::new().unwrap();
        let future = DefaultCpuTaskQueue.async_call(|| 1 + 1);
        assert_eq!(executor.run_future(future).unwrap(), Ok(2));

        let future = DefaultIoTaskQueue.async_call(|| -> () { panic!() });
        assert_eq!(executor.run_future(future).unwrap(), Err(AsyncCallError));
    }
}
