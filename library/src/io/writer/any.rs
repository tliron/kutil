use super::super::super::std::any::*;

use std::{any::*, io};

//
// AnyWriter
//

/// [io::Write] that can be converted to an [Any].
pub trait AnyWriter: ToAny + io::Write {}

/// Common reference type for [AnyWriter].
pub type AnyWriterRef = Box<dyn AnyWriter>;

//
// AnySeekWriter
//

/// [io::Seek] + [io::Write] that can be converted to an [Any].
pub trait AnySeekWriter: ToAny + io::Seek + io::Write {}

/// Common reference type for [AnySeekWriter].
pub type AnySeekWriterRef = Box<dyn AnySeekWriter>;

//
// AnyWriterWrapper
//

/// Wrapper that implements [AnyWriter] and [AnySeekWriter].
pub struct AnyWriterWrapper<WriteT> {
    /// Inner.
    pub inner: Option<WriteT>,
}

impl<WriteT> AnyWriterWrapper<WriteT> {
    /// Constructor.
    pub fn new(inner: WriteT) -> Box<Self> {
        Box::new(Self { inner: Some(inner) })
    }
}

impl<WriteT> ToAny for AnyWriterWrapper<WriteT>
where
    WriteT: Any,
{
    fn to_any(&mut self) -> Option<Box<dyn Any>> {
        // We defined inner as Option to make it simple to "take" it
        // It's just replaced with None whereas std::mem::take would need to replace it with WriteT::default()
        self.inner.take().map(|inner| Box::new(inner) as Box<dyn Any>)
    }
}

impl<WriteT> AnyWriter for AnyWriterWrapper<WriteT> where WriteT: Any + io::Write {}

impl<WriteT> AnySeekWriter for AnyWriterWrapper<WriteT> where WriteT: Any + io::Seek + io::Write {}

impl<WriteT> io::Seek for AnyWriterWrapper<WriteT>
where
    WriteT: io::Seek,
{
    fn seek(&mut self, from: io::SeekFrom) -> io::Result<u64> {
        self.inner.as_mut().expect("initialized").seek(from)
    }
}

impl<WriteT> io::Write for AnyWriterWrapper<WriteT>
where
    WriteT: io::Write,
{
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        self.inner.as_mut().expect("initialized").write(buffer)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.as_mut().expect("initialized").flush()
    }
}

//
// IntoAnyWriter
//

/// Convert into an [AnyWriter] and [AnySeekWriter] implementation.
pub trait IntoAnyWriter<WriteT> {
    /// Convert into an [AnyWriter] and [AnySeekWriter] implementation.
    fn into_any_writer(self) -> Box<AnyWriterWrapper<WriteT>>;
}

impl<WriteT> IntoAnyWriter<WriteT> for WriteT
where
    WriteT: Any + io::Write,
{
    fn into_any_writer(self) -> Box<AnyWriterWrapper<WriteT>> {
        AnyWriterWrapper::new(self)
    }
}
