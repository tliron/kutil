use std::{io, path::*};

//
// WithPath
//

/// With path.
pub trait WithPath {
    /// With path.
    fn with_path<PathT>(self, path: PathT) -> Self
    where
        PathT: AsRef<Path>;
}

impl WithPath for io::Error {
    fn with_path<PathT>(self, path: PathT) -> Self
    where
        PathT: AsRef<Path>,
    {
        Self::new(self.kind(), format!("{}: {}", self, path.as_ref().display()))
    }
}

impl<OkT> WithPath for io::Result<OkT> {
    fn with_path<PathT>(self, path: PathT) -> Self
    where
        PathT: AsRef<Path>,
    {
        self.map_err(|error| error.with_path(path))
    }
}
