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
    use futures::{Async, Future};
    use super::*;

    #[test]
    fn it_works() {
        let future = DefaultCpuTaskQueue.async_call(|| 1 + 1);
        assert_eq!(wait(future), Ok(2));

        let future = DefaultIoTaskQueue.async_call(|| -> () { panic!() });
        assert_eq!(wait(future), Err(AsyncCallError));
    }

    fn wait<F: Future>(mut f: F) -> Result<F::Item, F::Error> {
        loop {
            if let Async::Ready(v) = f.poll()? {
                return Ok(v);
            }
        }
    }
}
