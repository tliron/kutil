use super::query_map::*;

use {
    http::uri::*,
    std::{borrow::*, collections::*},
    url::form_urlencoded,
    urlencoding::*,
};

//
// PathAndQueryUtilities
//

/// [PathAndQuery] utilities.
pub trait PathAndQueryUtilities {
    /// Decoded path.
    fn decoded_path(&self) -> Option<Cow<'_, str>>;

    /// Decoded query.
    fn decoded_query(&self) -> Option<Vec<(Cow<'_, str>, Cow<'_, str>)>>;

    /// Decoded query as map.
    fn decoded_query_map(&self) -> Option<QueryMap> {
        if let Some(decoded_query) = self.decoded_query() {
            let mut map = QueryMap::default();

            for (name, value) in decoded_query {
                let name = name.as_ref();
                match map.get_mut(name) {
                    Some(values) => {
                        values.insert(value.into_owned().into());
                    }

                    None => {
                        let mut values = BTreeSet::default();
                        values.insert(value.into_owned().into());
                        map.insert(name.into(), values);
                    }
                }
            }

            return Some(map);
        }

        None
    }
}

impl PathAndQueryUtilities for PathAndQuery {
    fn decoded_path(&self) -> Option<Cow<'_, str>> {
        decode(self.path()).ok()
    }

    fn decoded_query(&self) -> Option<Vec<(Cow<'_, str>, Cow<'_, str>)>> {
        self.query().map(|query| form_urlencoded::parse(query.as_bytes()).collect())
    }
}

impl PathAndQueryUtilities for Uri {
    fn decoded_path(&self) -> Option<Cow<'_, str>> {
        self.path_and_query().and_then(|path_and_query| path_and_query.decoded_path())
    }

    fn decoded_query(&self) -> Option<Vec<(Cow<'_, str>, Cow<'_, str>)>> {
        self.path_and_query().and_then(|path_and_query| path_and_query.decoded_query())
    }
}
