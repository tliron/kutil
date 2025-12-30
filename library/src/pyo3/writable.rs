use pyo3::{intern, prelude::*};

/// Whether a file-like object is seekable.
pub fn is_file_like_seekable(writer: &Bound<'_, PyAny>) -> PyResult<bool> {
    writer.getattr(intern!(writer.py(), "seekable"))?.call0()?.extract()
}
