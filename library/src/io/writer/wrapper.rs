use std::{fmt, io};

//
// WriterWrapper
//

/// Writer wrapper.
pub struct WriterWrapper<WriteT> {
    /// Inner.
    pub inner: WriteT,
}

impl<WriteT> WriterWrapper<WriteT> {
    /// Constructor.
    pub fn new(inner: WriteT) -> Self {
        Self { inner }
    }
}

impl<WriteT> fmt::Write for WriterWrapper<WriteT>
where
    WriteT: io::Write,
{
    fn write_str(&mut self, string: &str) -> fmt::Result {
        self.inner.write_all(string.as_bytes()).map_err(|_| fmt::Error)
    }
}

//
// AsFmtWrite
//

/// To [fmt::Write] implementation.
pub trait AsFmtWrite
where
    Self: Sized,
{
    /// To [fmt::Write] implementation.
    fn as_fmt_write(self) -> WriterWrapper<Self>;
}

impl<WriteT> AsFmtWrite for WriteT
where
    WriteT: io::Write,
{
    fn as_fmt_write(self) -> WriterWrapper<Self> {
        WriterWrapper::new(self)
    }
}
