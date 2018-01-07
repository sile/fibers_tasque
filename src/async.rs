use std::fmt;
use std::error::Error;
use fibers::sync::oneshot;
use futures::{Future, Poll};
use tasque::TaskQueue;

/// [`TaskQueue`] extention.
///
/// [`TaskQueue`]: https://docs.rs/tasque/0.1/tasque/struct.TaskQueue.html
pub trait TaskQueueExt {
    /// Executes the given function asynchronously.
    ///
    /// The function `f` will be executed by a worker thread managed by this task queue.
    fn async_call<F, T>(&self, f: F) -> AsyncCall<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static;
}
impl TaskQueueExt for TaskQueue {
    fn async_call<F, T>(&self, f: F) -> AsyncCall<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        let (tx, rx) = oneshot::channel();
        self.enqueue(|| {
            let _ = tx.send(f());
        });
        AsyncCall(rx)
    }
}

/// A [`Future`] that represents an asynchronous function call.
///
/// This is created by calling `TaskQueueExt::async_call` function.
///
/// [`Future`]: https://docs.rs/futures/0.1/futures/future/trait.Future.html
#[derive(Debug)]
pub struct AsyncCall<T>(oneshot::Receiver<T>);
impl<T> Future for AsyncCall<T> {
    type Item = T;
    type Error = AsyncCallError;
    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.0.poll().map_err(|_| AsyncCallError)
    }
}

/// An [`Error`] used if the worker thread that executing a asynchronous function aborted.
///
/// [`Error`]: https://doc.rust-lang.org/std/error/trait.Error.html
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AsyncCallError;
impl fmt::Display for AsyncCallError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.description().fmt(f)
    }
}
impl Error for AsyncCallError {
    fn description(&self) -> &str {
        "a worker thread executing an asynchronous function call aborted"
    }
}
