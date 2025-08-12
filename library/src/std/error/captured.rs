use std::error::Error;

//
// CapturedError
//

/// A thread-safe `dyn` error.
///
/// Returning it as an error allows a function to gloss over actual error types. While not usually
/// desirable, it's a simple or even necessary solution for generic code in which internal error
/// types are not and cannot be known at compile time.
///
/// For generic returned errors it can be useful to specify [Into]\<[CapturedError]\> as a bound.
/// (Rust doesn't support trait aliases, so you have to spell it out.) This allows for implicit
/// error conversion, e.g. using the `?` operator.
///
/// Boxing errors is mentioned in
/// [Rust by Example](https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/boxing_errors.html),
/// though the example there is not thread-safe.
///
/// Other implementations add thread-safety, e.g.
/// [BoxError](https://docs.rs/tower-http/latest/tower_http/type.BoxError.html)
/// in Tower HTTP, which is the inspiration for this.
///
/// A far more comprehensive solution is provided by [anyhow](https://docs.rs/anyhow/latest/anyhow/).
pub type CapturedError = Box<dyn Error + Send + Sync>;

//
// BoxedError
//

/// A non-thread-safe (less constrained) version of [CapturedError].
pub type BoxedError = Box<dyn Error>;
