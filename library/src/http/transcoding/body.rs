use super::super::super::{
    std::{error::*, immutable::*},
    transcoding::{reader::*, *},
};

use super::super::body::*;

use {
    async_compression::*,
    http::*,
    http_body::*,
    pin_project::*,
    std::{collections::*, io, pin::*, result::Result, task::*},
    tokio_util::io::*,
};

const BUFFER_INITIAL_CAPACITY: usize = 8 * 1_024; // 8 KiB

//
// TranscodingBody
//

/// [Body] wrapper that can encode, decode, or pass through.
///
/// Note that the resulting number (and of course sizes) of the data frames will not necessarily
/// match those of the wrapped body.
///
/// Relies on [TranscodingReader].
#[pin_project]
pub struct TranscodingBody<InnerBodyT>
where
    InnerBodyT: Body,
    InnerBodyT::Error: Into<CapturedError>,
{
    #[pin]
    reader: TranscodingReader<BodyReader<InnerBodyT>>,
    buffer: BytesMut,
    trailers: Option<VecDeque<HeaderMap>>,
}

impl<InnerBodyT> TranscodingBody<InnerBodyT>
where
    InnerBodyT: Body,
    InnerBodyT::Error: Into<CapturedError>,
{
    /// Constructor.
    pub fn new(reader: TranscodingReader<BodyReader<InnerBodyT>>) -> Self {
        Self { reader, buffer: BytesMut::with_capacity(0), trailers: None }
    }

    fn validate_buffer_capacity(&mut self) {
        let capacity = self.buffer.capacity();
        if capacity < BUFFER_INITIAL_CAPACITY {
            self.buffer.reserve(BUFFER_INITIAL_CAPACITY - capacity);
        }
    }
}

impl<BodyT> From<Bytes> for TranscodingBody<BodyT>
where
    BodyT: Body + From<Bytes>,
    BodyT::Error: Into<CapturedError>,
{
    fn from(bytes: Bytes) -> Self {
        let body: BodyT = bytes.into();
        body.into_transcoding_passthrough_with_first_bytes(None)
    }
}

impl<InnerBodyT> Body for TranscodingBody<InnerBodyT>
where
    InnerBodyT: Body,
    InnerBodyT::Data: From<Bytes>,
    InnerBodyT::Error: Into<CapturedError>,
{
    type Data = InnerBodyT::Data;
    type Error = io::Error;

    fn poll_frame(
        mut self: Pin<&mut Self>,
        context: &mut Context<'_>,
    ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
        // Return remaining buffer as data frame
        if self.buffer.has_remaining() {
            let bytes = self.buffer.split().freeze();
            let frame = Frame::data(bytes.into());
            return Poll::Ready(Some(Ok(frame)));
        }

        self.validate_buffer_capacity();

        let projected_self = self.as_mut().project();

        Poll::Ready({
            let count = ready!(poll_read_buf(projected_self.reader, context, projected_self.buffer))?;

            if count != 0 {
                let bytes = projected_self.buffer.split_to(count).freeze();
                let frame = Frame::data(bytes.into());
                Some(Ok(frame))
            } else {
                // count = 0 means we are done with data

                // Make sure we have the trailers
                if self.trailers.is_none() {
                    let trailers = &self.reader.inner().trailers;
                    if !trailers.is_empty() {
                        self.trailers = Some(trailers.clone().into());
                    }
                }

                // Return the next trailer frame
                self.trailers
                    .as_mut()
                    .and_then(|trailers| trailers.pop_front().map(|trailers| Ok(Frame::trailers(trailers))))
            }
        })
    }
}

//
// IntoTranscodingBody
//

/// Into a [TranscodingBody].
pub trait IntoTranscodingBody<BodyT>
where
    Self: Sized,
    BodyT: Body,
    BodyT::Error: Into<CapturedError>,
{
    /// Into passthrough [TranscodingBody].
    fn into_transcoding_passthrough(self) -> TranscodingBody<BodyT> {
        self.into_transcoding_passthrough_with_first_bytes(None)
    }

    /// Into passthrough [TranscodingBody].
    fn into_transcoding_passthrough_with_first_bytes(self, first_bytes: Option<Bytes>) -> TranscodingBody<BodyT>;

    /// Into encoding [TranscodingBody].
    fn into_encoding(self, encoding: &Encoding) -> TranscodingBody<BodyT> {
        self.into_encoding_with_first_bytes(None, encoding)
    }

    /// Into encoding [TranscodingBody].
    fn into_encoding_with_first_bytes(self, first_bytes: Option<Bytes>, encoding: &Encoding) -> TranscodingBody<BodyT>;

    /// Into decoding [TranscodingBody].
    fn into_decoding(self, encoding: &Encoding) -> TranscodingBody<BodyT> {
        self.into_decoding_with_first_bytes(None, encoding)
    }

    /// Into decoding [TranscodingBody].
    fn into_decoding_with_first_bytes(self, first_bytes: Option<Bytes>, encoding: &Encoding) -> TranscodingBody<BodyT>;
}

impl<BodyT> IntoTranscodingBody<BodyT> for BodyT
where
    BodyT: Body,
    BodyT::Error: Into<CapturedError>,
{
    fn into_transcoding_passthrough_with_first_bytes(self, first_bytes: Option<Bytes>) -> TranscodingBody<BodyT> {
        TranscodingBody::new(self.into_reader_with_first_bytes(first_bytes).into_passthrough_reader())
    }

    fn into_encoding_with_first_bytes(self, first_bytes: Option<Bytes>, encoding: &Encoding) -> TranscodingBody<BodyT> {
        TranscodingBody::new(
            self.into_reader_with_first_bytes(first_bytes).into_encoding_reader(encoding, Level::Fastest),
        )
    }

    fn into_decoding_with_first_bytes(self, first_bytes: Option<Bytes>, encoding: &Encoding) -> TranscodingBody<BodyT> {
        TranscodingBody::new(self.into_reader_with_first_bytes(first_bytes).into_decoding_reader(encoding))
    }
}
