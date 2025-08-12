use std::{io::*, sync::*};

//
// ReadableBuffer
//

/// An immutable buffer that can be read concurrently.
///
/// It's simply a buffer wrapped in an [Arc] and that can match the requirements of
/// [Cursor].
#[derive(Clone, Debug)]
pub struct ReadableBuffer {
    buffer: Arc<[u8]>,
}

/// [ReadableBuffer] reader.
pub type ReadableBufferReader = Cursor<ReadableBuffer>;

impl ReadableBuffer {
    /// Constructor.
    pub fn new(buffer: &[u8]) -> Self {
        Self { buffer: Arc::from(buffer) }
    }

    /// Reader.
    pub fn reader(&self) -> ReadableBufferReader {
        // We are cloning the Arc, not the buffer
        ReadableBufferReader::new(self.clone())
    }
}

impl AsRef<[u8]> for ReadableBuffer {
    fn as_ref(&self) -> &[u8] {
        &self.buffer
    }
}
