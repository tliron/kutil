use super::uri::*;

use {
    http::{uri::*, *},
    std::result::Result,
};

//
// SetUri
//

/// Set [Uri].
pub trait SetUri {
    /// Set [Uri].
    fn set_uri(&mut self, uri: Uri);

    /// Set [Uri] path.
    fn set_uri_path(&mut self, path: &str) -> Result<(), Error>;
}

impl<BodyT> SetUri for Request<BodyT> {
    fn set_uri(&mut self, uri: Uri) {
        *self.uri_mut() = uri;
    }

    fn set_uri_path(&mut self, path: &str) -> Result<(), Error> {
        let uri = self.uri().with_path(path)?;
        self.set_uri(uri);
        Ok(())
    }
}
