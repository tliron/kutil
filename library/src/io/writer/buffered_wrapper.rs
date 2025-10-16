#![allow(unused)]

use std::{fmt, io, str::*};

//
// BufferedWriterWrapper
//

/// Buffered writer wrapper.
pub struct BufferedWriterWrapper<WriteT> {
    /// Inner.
    pub inner: WriteT,

    /// Remainder.
    pub remainder: Option<Vec<u8>>,
}

impl<WriteT> BufferedWriterWrapper<WriteT> {
    /// Constructor.
    pub fn new(inner: WriteT) -> Self {
        Self { inner, remainder: None }
    }
}

impl<WriteT> fmt::Write for BufferedWriterWrapper<WriteT>
where
    WriteT: io::Write,
{
    fn write_str(&mut self, string: &str) -> fmt::Result {
        self.inner.write_all(string.as_bytes()).map_err(|_| fmt::Error)
    }
}

impl<WriteT> io::Write for BufferedWriterWrapper<WriteT>
where
    WriteT: fmt::Write,
{
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        // TODO:
        // This is bad :(
        // Would we just be increasing the remainder forever if there are constant errors?
        // What would happen with a leftover remainder?
        // This is likely unsolvable...

        let mut _buffer = Vec::default();

        let buffer = match self.remainder.take() {
            Some(remainder) => {
                _buffer = remainder;
                _buffer.extend(buffer);
                &_buffer
            }

            None => buffer,
        };

        let string = match from_utf8(buffer) {
            Ok(string) => string,

            Err(error) => {
                let up_to = error.valid_up_to();

                let remainder_start = up_to + 1;
                if remainder_start < buffer.len() {
                    self.remainder = Some(buffer[remainder_start..].to_vec());
                }

                if up_to == 0 {
                    return Ok(0);
                }

                match from_utf8(&buffer[..up_to]) {
                    Ok(string) => string,

                    // This should never happen!
                    Err(error) => return Err(io::Error::new(io::ErrorKind::InvalidData, error)),
                }
            }
        };

        self.inner.write_str(string).map(|_| string.len()).map_err(io::Error::other)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
