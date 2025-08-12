use std::io;

//
// BoundedReader
//

/// Bounded [io::Read] wrapper.
pub struct BoundedReader<InnerT>
where
    InnerT: io::Read,
{
    /// Inner.
    pub inner: InnerT,

    /// Max size.
    pub max_size: usize,

    completed: usize,
}

impl<ReadT> BoundedReader<ReadT>
where
    ReadT: io::Read,
{
    /// Constructor.
    pub fn new(inner: ReadT, max_size: usize) -> Self {
        Self { inner, max_size, completed: 0 }
    }
}

impl<InnerT> io::Read for BoundedReader<InnerT>
where
    InnerT: io::Read,
{
    fn read(&mut self, mut buf: &mut [u8]) -> io::Result<usize> {
        let mut buf_len = buf.len();
        if buf_len == 0 {
            return Ok(0);
        }

        // What we want
        let end = self.completed + buf_len;

        // What we can do
        if end > self.max_size {
            buf_len = self.max_size - self.completed;
            if buf_len == 0 {
                // Note: this is not just an optimization:
                // some readers fail when given empty buffers
                // See: https://github.com/gyscos/zstd-rs/issues/318
                return Ok(0);
            }
            buf = &mut buf[..buf_len];
        }

        let completed = self.inner.read(buf)?;
        self.completed += completed;
        return Ok(completed);
    }
}
