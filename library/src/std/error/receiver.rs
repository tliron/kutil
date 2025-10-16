//
// ErrorReceiver
//

/// A receiver of errors.
///
/// Example of usage:
///
/// ```
/// fn divide<ErrorReceiverT>(a: f64, b: f64, errors: &mut ErrorReceiverT) -> Result<Option<f64>, String>
/// where
///     ErrorReceiverT: ErrorReceiver<String>,
/// {
///     Ok(if b == 0.0 {
///         errors.give("division by zero")?;
///         None
///     } else {
///         Some(a / b)
///     })
/// }
/// ```
///
/// If a generic type of this trait cannot be, e.g. within a `dyn`-compatible trait, then consider
/// using [ErrorReceiverRef](super::receiver_ref::ErrorReceiverRef) instead.
pub trait ErrorReceiver<ErrorT> {
    /// Gives an error to the receiver.
    ///
    /// Implementations may swallow the error (e.g. to accumulate it) or return it (fail-fast).
    fn give_error(&mut self, error: ErrorT) -> Result<(), ErrorT>;
}

//
// Give
//

/// Gives an error to the receiver.
pub trait Give<ErrorT> {
    /// Gives an error to the receiver.
    ///
    /// Implementations may swallow the error (e.g. to accumulate it) or return it (fail-fast).
    fn give(&mut self, error: impl Into<ErrorT>) -> Result<(), ErrorT>;
}

impl<ErrorT, ErrorReceiverT> Give<ErrorT> for ErrorReceiverT
where
    ErrorReceiverT: ErrorReceiver<ErrorT>,
{
    fn give(&mut self, error: impl Into<ErrorT>) -> Result<(), ErrorT> {
        self.give_error(error.into())
    }
}
