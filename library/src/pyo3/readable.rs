use {
    pyo3::prelude::*,
    std::{
        borrow::*,
        io::{self, Read},
        str::*,
    },
};

#[allow(unused_imports)]
pub use {pyo3_filelike::PyBinaryFile, pyo3_filelike::PyTextFile};

//
// ReadableAny
//

/// Readable [PyAny].
pub enum ReadableAny<'this> {
    /// Bytes.
    Bytes(&'this [u8]),

    /// String.
    String(&'this str),

    /// File-like.
    FileLike(PyBinaryFile),
}

impl<'this> ReadableAny<'this> {
    /// Into bytes.
    pub fn into_bytes(self) -> io::Result<Cow<'this, [u8]>> {
        Ok(match self {
            Self::Bytes(bytes) => bytes.into(),
            Self::String(string) => string.as_bytes().into(),
            Self::FileLike(mut file_like) => {
                let mut buffer = Vec::default();
                file_like.read_to_end(&mut buffer)?;
                buffer.into()
            }
        })
    }

    /// Into string.
    pub fn into_string(self) -> io::Result<Cow<'this, str>> {
        Ok(match self {
            Self::Bytes(bytes) => from_utf8(bytes).map_err(io::Error::other)?.into(),
            Self::String(string) => string.into(),
            Self::FileLike(file_like) => io::read_to_string(file_like)?.into(),
        })
    }
}

impl<'bound, 'py> From<&'bound Bound<'py, PyAny>> for ReadableAny<'bound> {
    fn from(any: &'bound Bound<'py, PyAny>) -> Self {
        if let Ok(bytes) = any.extract() {
            Self::Bytes(bytes)
        } else if let Ok(string) = any.extract() {
            Self::String(string)
        } else {
            // Bound::clone() is a cheap clone_ref
            Self::FileLike(PyBinaryFile::from(any.clone()))
        }
    }
}
