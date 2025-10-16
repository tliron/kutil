/// [Any](std::any::Any) utilities.
pub mod any;

/// Borrow utilities.
pub mod borrow;

/// Collections utilities.
pub mod collections;

/// Error utilities.
pub mod error;

/// Exit utilities.
pub mod exit;

/// Foster utilities.
pub mod foster;

/// Filesystem utilities.
pub mod fs;

/// Future utilities.
pub mod future;

/// Iteration utilities.
pub mod iter;

/// Metric utilities.
pub mod metric;

/// Scope utilities.
pub mod scope;

/// String utilities.
pub mod string;

/// Synchronization utilities.
pub mod sync;

/// Time utilities.
pub mod time;

/// Immutable ("zero-copy") utilities.
#[cfg(feature = "immutable")]
#[allow(unused_imports)]
pub mod immutable;

#[cfg(feature = "derive")]
#[allow(unused_imports)]
pub use kutil_std_macros::*;
