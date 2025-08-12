use super::super::std::*;

/// Encodings in order from cheapest to decode to most expensive.
pub const ENCODINGS_BY_DECODING_COST: &[Encoding] =
    &[Encoding::Zstandard, Encoding::Deflate, Encoding::GZip, Encoding::Brotli];

//
// Encoding
//

/// HTTP encoding.
#[derive(Clone, Copy, Debug, Default, Display, Eq, Hash, PartialEq)]
pub enum Encoding {
    /// Identity.
    #[default]
    Identity,

    /// Brotli.
    Brotli,

    /// Deflate.
    Deflate,

    /// GZip.
    GZip,

    /// Zstandard.
    Zstandard,
}
