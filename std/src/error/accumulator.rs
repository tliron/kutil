use super::{errors::*, recipient::*};

//
// ErrorAccumulator
//

/// An [ErrorRecipient] that can either fail-fast or accumulate.
///
/// Note that unlike [ErrorRecipient], which is a trait, this is a struct and as such has only its
/// defined behavior. To allow for more flexibility follow the example in [ErrorRecipient] instead
/// of using this type.
pub enum ErrorAccumulator<'errors, ErrorT> {
    /// Fail on the first given error.
    FailFast,

    /// Accumulate errors.
    Accumulate(&'errors mut Errors<ErrorT>),
}

impl<'errors, ErrorT> ErrorAccumulator<'errors, ErrorT> {
    /// Constructor.
    pub fn new(errors: &'errors mut Errors<ErrorT>) -> Self {
        Self::Accumulate(errors)
    }

    /// Create a new [ErrorAccumulator] with the same mode.
    ///
    /// If we're [FailFast](ErrorAccumulator::FailFast), it will also be
    /// [FailFast](ErrorAccumulator::FailFast) and the "errors" argument will be ignored.
    pub fn new_like<'newerrors, NewErrorT>(
        &self,
        errors: &'newerrors mut Errors<NewErrorT>,
    ) -> ErrorAccumulator<'newerrors, NewErrorT> {
        match self {
            Self::FailFast => ErrorAccumulator::FailFast,
            Self::Accumulate(_) => errors.as_accumulator(),
        }
    }
}

impl<'errors, ErrorT> ErrorRecipient<ErrorT> for ErrorAccumulator<'errors, ErrorT> {
    fn give(&mut self, error: impl Into<ErrorT>) -> Result<(), ErrorT> {
        match self {
            Self::FailFast => Err(error.into()),
            Self::Accumulate(errors) => errors.give(error),
        }
    }
}
