//
// ErrorRecipient
//

/// A recipient of errors.
///
/// Example of usage:
///
/// ```
/// fn divide<ErrorRecipientT>(a: f64, b: f64, errors: &mut ErrorRecipientT) -> Result<Option<f64>, String>
/// where
///     ErrorRecipientT: ErrorRecipient<String>,
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
/// using [ErrorRecipientRef](super::recipient_ref::ErrorRecipientRef) instead.
pub trait ErrorRecipient<ErrorT> {
    /// Gives an error to the recipient.
    ///
    /// Implementations may swallow the error (e.g. to accumulate it) or return it (fail-fast).
    fn give_error(&mut self, error: ErrorT) -> Result<(), ErrorT>;
}

//
// Give
//

/// Gives an error to the recipient.
pub trait Give<ErrorT> {
    /// Gives an error to the recipient.
    ///
    /// Implementations may swallow the error (e.g. to accumulate it) or return it (fail-fast).
    fn give(&mut self, error: impl Into<ErrorT>) -> Result<(), ErrorT>;
}

impl<ErrorT, ErrorRecipientT> Give<ErrorT> for ErrorRecipientT
where
    ErrorRecipientT: ErrorRecipient<ErrorT>,
{
    fn give(&mut self, error: impl Into<ErrorT>) -> Result<(), ErrorT> {
        self.give_error(error.into())
    }
}
