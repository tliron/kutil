use std::{io, pin::*};

/// A thread-safe and pinned `dyn` [Future].
///
/// Returning a [CapturedFuture] can allow us to call `async` code in a non-async function,
/// e.g. the polling function for futures, readers, writers, etc.
///
/// See also [capture_async].
pub type CapturedFuture<OutputT> = Pin<Box<dyn Future<Output = OutputT> + Send>>;

/// Captures async code into a [CapturedFuture].
///
/// This works by wrapping the code in `Box::pin(async move { ... } )`.
#[macro_export]
macro_rules! capture_async {
    ( $( $code:tt )* ) => {
        ::std::boxed::Box::pin(async move { $( $code )* })
    };
}

#[allow(unused_imports)]
pub use capture_async;

/// A [CapturedFuture] for I/O tasks.
pub type CapturedIoTask = CapturedFuture<io::Result<()>>;
