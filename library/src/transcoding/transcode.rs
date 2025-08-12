use super::encoding::*;

use std::io;

//
// Transcode
//

/// Transcode.
#[allow(async_fn_in_trait)]
pub trait Transcode
where
    Self: Sized,
{
    /// Encode.
    async fn encode(&self, encoding: &Encoding) -> io::Result<Self>;

    /// Decode.
    async fn decode(&self, encoding: &Encoding) -> io::Result<Self>;
}
