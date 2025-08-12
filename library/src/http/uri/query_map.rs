use super::super::super::std::immutable::*;

use std::collections::*;

//
// QueryMap
//

/// URI query as map.
pub type QueryMap = BTreeMap<ByteString, BTreeSet<ByteString>>;

//
// QueryMapUtilities
//

/// [QueryMap] utilities.
pub trait QueryMapUtilities {
    /// Gets a value for a key *only* if there is a single value.
    fn get_single(&self, key: &str) -> Option<&ByteString>;

    /// Gets a value for a key *only* if there is a single value.
    fn get_single_as_ref(&self, key: &str) -> Option<&str> {
        self.get_single(key).map(|value| value.as_ref())
    }
}

impl QueryMapUtilities for QueryMap {
    fn get_single(&self, key: &str) -> Option<&ByteString> {
        let values = self.get(key)?;
        match values.len() {
            1 => values.first(),
            _ => None,
        }
    }
}
