use super::path_and_query::*;

use {
    http::{uri::*, *},
    std::{result::Result, string::*},
};

//
// UriUtilities
//

/// [Uri] utilities.
pub trait UriUtilities: PathAndQueryUtilities {
    /// With new path.
    fn with_path(&self, path: &str) -> Result<Uri, Error>;
}

impl UriUtilities for Uri {
    fn with_path(&self, path: &str) -> Result<Uri, Error> {
        //let mut path_and_query = encode(path).into_owned();
        let mut path_and_query = String::from(path);
        if let Some(query) = self.query() {
            path_and_query = path_and_query + "?" + query;
        }

        let mut builder = Self::builder().path_and_query(path_and_query);

        if let Some(scheme) = self.scheme() {
            builder = builder.scheme(scheme.clone());
        }

        if let Some(authority) = self.authority() {
            builder = builder.authority(authority.clone());
        }

        builder.build()
    }
}
