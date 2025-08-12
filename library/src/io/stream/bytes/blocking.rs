use super::super::{
    super::super::std::{error::*, immutable::*},
    blocking::*,
};

use {
    futures::*,
    std::{cmp::*, io},
};

const REMAINDER_INITIAL_CAPACITY: usize = 8 * 1_024; // 8 KiB

//
// BlockingBytesStreamReader
//

/// A [Read](io::Read) implementation for a [Stream] of [Result]\<[Bytes], _\>.
///
/// Errors are wrapped as [io::ErrorKind::Other].
///
/// Useful, for example, for reading from
/// [reqwest::Response::byte_stream](https://github.com/seanmonstar/reqwest).
pub struct BlockingBytesStreamReader<StreamT, ErrorT>
where
    StreamT: Stream<Item = Result<Bytes, ErrorT>> + Unpin,
{
    stream: BlockingStream<StreamT>,

    /// Remainder.
    pub remainder: BytesMut,
}

impl<StreamT, ErrorT> BlockingBytesStreamReader<StreamT, ErrorT>
where
    StreamT: Stream<Item = Result<Bytes, ErrorT>> + Unpin,
    ErrorT: Into<CapturedError>,
{
    /// Constructor.
    pub fn new(stream: BlockingStream<StreamT>) -> Self {
        Self { stream, remainder: BytesMut::with_capacity(0) }
    }

    /// Back to the inner [Stream].
    ///
    /// Note that the stream may have changed if we have read from this reader, in which case the
    /// returned remainder will be non-empty.
    pub fn into_inner(self) -> (BlockingStream<StreamT>, BytesMut) {
        (self.stream, self.remainder)
    }

    fn validate_remainder_capacity(&mut self) {
        let capacity = self.remainder.capacity();
        if capacity < REMAINDER_INITIAL_CAPACITY {
            self.remainder.reserve(REMAINDER_INITIAL_CAPACITY - capacity);
        }
    }
}

impl<StreamT, ErrorT> io::Read for BlockingBytesStreamReader<StreamT, ErrorT>
where
    StreamT: Stream<Item = Result<Bytes, ErrorT>> + Unpin,
    ErrorT: Into<CapturedError>,
{
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        let mut buffer_position = 0;
        let mut buffer_remaining = buffer.len();

        // Copy as much as we can from the remainder
        if self.remainder.has_remaining() {
            let size = min(buffer_remaining, self.remainder.remaining());

            if size != 0 {
                self.remainder.copy_to_slice(&mut buffer[..size]);

                if size == buffer_remaining {
                    // Buffer is full
                    return Ok(size);
                }

                buffer_position = size;
                buffer_remaining -= size;
            }
        }

        match self.stream.next() {
            Some(result) => {
                let mut bytes = result.map_err(io::Error::other)?;

                // Copy as much as we can from the bytes
                let size = min(buffer_remaining, bytes.remaining());

                if size != 0 {
                    bytes.copy_to_slice(&mut buffer[buffer_position..buffer_position + size]);
                }

                // Store leftover bytes in the remainder
                if bytes.has_remaining() {
                    self.validate_remainder_capacity();
                    self.remainder.put(bytes);
                }

                Ok(buffer_position + size)
            }

            None => Ok(buffer_position),
        }
    }
}
