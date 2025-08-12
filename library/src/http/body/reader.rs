use super::super::super::std::{error::*, immutable::*};

use {
    http::*,
    http_body::*,
    std::{cmp::*, io, pin::*, task::*},
    tokio::io::*,
};

const REMAINDER_INITIAL_CAPACITY: usize = 8 * 1_024; // 8 KiB

//
// BodyReader
//

/// [AsyncRead] wrapper for [Body].
pub struct BodyReader<BodyT> {
    body: Pin<Box<BodyT>>,

    /// Remainder.
    pub remainder: BytesMut,

    /// Trailers
    pub trailers: Vec<HeaderMap>,
}

impl<BodyT> BodyReader<BodyT> {
    /// Constructor.
    pub fn new(body: BodyT) -> Self {
        Self::new_with_first_bytes(body, None)
    }

    /// Constructor.
    pub fn new_with_first_bytes(body: BodyT, first_bytes: Option<Bytes>) -> Self {
        let remainder = match first_bytes {
            Some(first_bytes) => first_bytes.into(),
            None => BytesMut::with_capacity(0),
        };

        Self { body: Box::pin(body), remainder, trailers: Default::default() }
    }

    /// Back to the inner [Body].
    ///
    /// Note that the body may have changed if we have read from this reader, in which case the
    /// returned remainder will be non-empty and/or we may have trailers.
    pub fn into_inner(self) -> (BodyT, BytesMut, Vec<HeaderMap>)
    where
        BodyT: Unpin,
    {
        (*Pin::into_inner(self.body), self.remainder, self.trailers)
    }

    fn validate_remainder_capacity(&mut self) {
        let capacity = self.remainder.capacity();
        if capacity < REMAINDER_INITIAL_CAPACITY {
            self.remainder.reserve(REMAINDER_INITIAL_CAPACITY - capacity);
        }
    }
}

impl<BodyT> AsyncRead for BodyReader<BodyT>
where
    BodyT: Body,
    BodyT::Error: Into<CapturedError>, // so it can be used with io::Error::other
{
    fn poll_read(
        mut self: Pin<&mut Self>,
        context: &mut Context<'_>,
        buffer: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        // Copy as much as we can from the remainder
        if self.remainder.has_remaining() {
            let size = min(buffer.remaining_mut(), self.remainder.remaining());

            if size != 0 {
                let bytes = self.remainder.copy_to_bytes(size);
                buffer.put(bytes);

                if !buffer.has_remaining_mut() {
                    // Buffer is full
                    return Poll::Ready(Ok(()));
                }
            }
        }

        Poll::Ready(match ready!(self.body.as_mut().poll_frame(context)) {
            Some(result) => {
                let frame = result.map_err(io::Error::other)?;
                match frame.into_data() {
                    Ok(mut data) => {
                        // Copy as much as we can from the data
                        let size = min(buffer.remaining_mut(), data.remaining());

                        if size != 0 {
                            let bytes = data.copy_to_bytes(size);
                            buffer.put(bytes);
                        }

                        // Store leftover data in the remainder
                        if data.has_remaining() {
                            self.validate_remainder_capacity();
                            self.remainder.put(data);
                        }

                        Ok(())
                    }

                    // Note that this is not actually an error
                    Err(frame) => {
                        match frame.into_trailers() {
                            Ok(trailers) => {
                                tracing::debug!("trailers frame");
                                self.trailers.push(trailers);

                                // Note: There really shouldn't be more than one trailers frame,
                                // but Body::poll_frame doesn't explicitly disallow it so we
                                // make sure to collect them all into a vector
                            }

                            Err(_frame) => {
                                tracing::warn!("frame is not data and not trailers");
                            }
                        }

                        Ok(())
                    }
                }
            }

            None => Ok(()),
        })
    }
}

//
// IntoBodyReader
//

/// Into [BodyReader].
pub trait IntoBodyReader<BodyT>
where
    Self: Sized,
{
    /// Into [BodyReader].
    fn into_reader(self) -> BodyReader<BodyT> {
        self.into_reader_with_first_bytes(None)
    }

    /// Into [BodyReader].
    fn into_reader_with_first_bytes(self, first_bytes: Option<Bytes>) -> BodyReader<BodyT>;
}

impl<BodyT> IntoBodyReader<BodyT> for BodyT
where
    BodyT: Body,
    BodyT::Error: Into<CapturedError>,
{
    fn into_reader_with_first_bytes(self, first_bytes: Option<Bytes>) -> BodyReader<BodyT> {
        BodyReader::new_with_first_bytes(self, first_bytes)
    }
}
