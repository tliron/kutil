use super::{super::super::std::immutable::*, body::*};

use {
    http::response::*,
    std::{error::*, fmt},
};

//
// ResponsePieces
//

/// [Response] pieces.
///
/// Can be used to reconstruct a response, e.g. with
/// [BodyReader::new_with_first_bytes](super::super::body::BodyReader::new_with_first_bytes).
#[derive(Clone, Debug)]
pub struct ResponsePieces<ResponseBodyT> {
    /// Response.
    pub response: Response<ResponseBodyT>,

    /// First bytes.
    pub first_bytes: Bytes,
}

impl<ResponseBodyT> ResponsePieces<ResponseBodyT> {
    /// Constructor.
    pub fn new(parts: Parts, body: ResponseBodyT, first_bytes: Bytes) -> Self {
        Self { response: Response::from_parts(parts, body), first_bytes }
    }

    /// Constructor.
    pub fn new_from_body_pieces(parts: Parts, body_pieces: BodyPieces<ResponseBodyT>) -> Self {
        Self::new(parts, body_pieces.body, body_pieces.first_bytes)
    }
}

//
// ErrorWithResponsePieces
//

/// [Error] with optional [ResponsePieces].
pub struct ErrorWithResponsePieces<ErrorT, BodyT> {
    /// Error.
    pub error: ErrorT,

    /// Pieces.
    pub pieces: Option<ResponsePieces<BodyT>>,
}

impl<ErrorT, BodyT> ErrorWithResponsePieces<ErrorT, BodyT> {
    /// Constructor.
    pub fn new(error: ErrorT, pieces: Option<ResponsePieces<BodyT>>) -> Self {
        Self { error, pieces }
    }

    /// Constructor.
    pub fn new_from_body(error: ErrorWithBodyPieces<ErrorT, BodyT>, parts: Parts) -> Self {
        Self::new(error.error, error.pieces.map(|pieces| ResponsePieces::new_from_body_pieces(parts, pieces)))
    }
}

impl<ErrorT, BodyT> fmt::Debug for ErrorWithResponsePieces<ErrorT, BodyT>
where
    ErrorT: fmt::Debug,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.error, formatter)
    }
}

impl<ErrorT, BodyT> fmt::Display for ErrorWithResponsePieces<ErrorT, BodyT>
where
    ErrorT: fmt::Display,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.error, formatter)
    }
}

impl<ErrorT, BodyT> Error for ErrorWithResponsePieces<ErrorT, BodyT>
where
    ErrorT: Error,
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.error.source()
    }
}

impl<ErrorT, BodyT> From<ErrorT> for ErrorWithResponsePieces<ErrorT, BodyT> {
    fn from(error: ErrorT) -> Self {
        Self::new(error, None)
    }
}
