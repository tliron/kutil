use super::super::super::std::immutable::*;

use std::{error::*, fmt};

//
// BodyPieces
//

/// [Body](http_body::Body) pieces.
///
/// Can be used to reconstruct a body, e.g. with
/// [BodyReader::new_with_first_bytes](super::super::body::BodyReader::new_with_first_bytes).
#[derive(Clone, Debug)]
pub struct BodyPieces<BodyT> {
    /// Body.
    pub body: BodyT,

    /// First bytes.
    pub first_bytes: Bytes,
}

impl<BodyT> BodyPieces<BodyT> {
    /// Constructor.
    pub fn new(body: BodyT, first_bytes: Bytes) -> Self {
        Self { body, first_bytes }
    }
}

//
// ErrorWithBodyPieces
//

/// [Error] with optional [BodyPieces].
pub struct ErrorWithBodyPieces<ErrorT, BodyT> {
    /// Error.
    pub error: ErrorT,

    /// Pieces.
    pub pieces: Option<BodyPieces<BodyT>>,
}

impl<ErrorT, BodyT> ErrorWithBodyPieces<ErrorT, BodyT> {
    /// Constructor.
    pub fn new(error: ErrorT, pieces: Option<BodyPieces<BodyT>>) -> Self {
        Self { error, pieces }
    }
}

impl<ErrorT, BodyT> fmt::Debug for ErrorWithBodyPieces<ErrorT, BodyT>
where
    ErrorT: fmt::Debug,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.error, formatter)
    }
}

impl<ErrorT, BodyT> fmt::Display for ErrorWithBodyPieces<ErrorT, BodyT>
where
    ErrorT: fmt::Display,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.error, formatter)
    }
}

impl<ErrorT, BodyT> Error for ErrorWithBodyPieces<ErrorT, BodyT>
where
    ErrorT: Error,
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.error.source()
    }
}

impl<ErrorT, BodyT> From<ErrorT> for ErrorWithBodyPieces<ErrorT, BodyT> {
    fn from(error: ErrorT) -> Self {
        Self::new(error, None)
    }
}
