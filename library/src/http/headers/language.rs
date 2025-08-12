use super::super::{
    super::std::{collections::*, foster::*},
    cache::*,
};

use {
    http::header::*,
    std::{convert::*, fmt, str::*},
};

//
// Language
//

/// Language tag value in HTTP headers.
///
/// See [IETF RFC 5646 section 2.1](https://datatracker.ietf.org/doc/html/rfc5646#section-2.1).
///
/// Stored as a sequence of subtags.
///
/// Note that even though ISO recommends cased representations, they are case-insensitive in HTTP.
/// Thus we convert all subtags to lowercase for comparison efficiency.
#[derive(Clone, Debug)]
pub struct Language(pub FosterByteStringVector);

delegate_newtype_of_foster_byte_string_vector!(Language);

impl Language {
    /// Parse list.
    pub fn parse_list(representation: &str) -> Option<FastHashSet<Self>> {
        let languages: FastHashSet<_> = representation.split(",").map(|language| language.trim().into()).collect();
        if !languages.is_empty() { Some(languages) } else { None }
    }
}

impl CacheWeight for Language {
    fn cache_weight(&self) -> usize {
        const SELF_SIZE: usize = size_of::<Language>();
        let mut size = SELF_SIZE;
        for subtag in &self.0 {
            size += subtag.len();
        }
        size
    }
}

impl Into<HeaderValue> for Language {
    fn into(self) -> HeaderValue {
        HeaderValue::from_str(&self.to_string()).expect("language in HTTP header")
    }
}

impl From<&str> for Language {
    fn from(representation: &str) -> Self {
        Self::new_owned(representation.split("-").map(|subtag| subtag.to_lowercase().into()).collect())
    }
}

impl FromStr for Language {
    type Err = Infallible;

    fn from_str(representation: &str) -> Result<Self, Self::Err> {
        Ok(representation.into())
    }
}

impl fmt::Display for Language {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            Foster::Owned(subtags) => fmt::Display::fmt(&subtags.join("-"), formatter),
            Foster::Fostered(subtags) => fmt::Display::fmt(&subtags.join("-"), formatter),
        }
    }
}
