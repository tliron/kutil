use super::captured::*;

use std::{error::Error, fmt};

//
// ManyBoxedErrors
//

/// Many [BoxedError].
pub struct ManyBoxedErrors {
    /// Errors.
    pub errors: Vec<BoxedError>,
}

impl From<Vec<BoxedError>> for ManyBoxedErrors {
    fn from(errors: Vec<BoxedError>) -> Self {
        Self { errors }
    }
}

impl fmt::Debug for ManyBoxedErrors {
    fn fmt(&self, _formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl fmt::Display for ManyBoxedErrors {
    fn fmt(&self, _formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl Error for ManyBoxedErrors {}

impl<ErrorT> FromIterator<ErrorT> for ManyBoxedErrors
where
    ErrorT: 'static + Error,
{
    fn from_iter<IntoIteratorT>(iterator: IntoIteratorT) -> Self
    where
        IntoIteratorT: IntoIterator<Item = ErrorT>,
    {
        iterator.into_iter().map(|error| error.into()).collect::<Vec<_>>().into()
    }
}
