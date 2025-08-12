use super::{
    super::{
        super::{super::std::immutable::*, headers::*, uri::*},
        weight::*,
    },
    key::*,
};

use {
    http::{header::*, uri::*, *},
    std::{collections::*, fmt, hash::*},
};

//
// CommonCacheKey
//

/// [CacheKey] implementation designed for common use cases.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct CommonCacheKey {
    /// Method.
    pub method: Method,

    /// Optional path.
    pub path: Option<ByteString>,

    /// Optional query (sorted by key).
    pub query: Option<QueryMap>,

    /// Optional scheme.
    ///
    /// Not set by default but reserved for custom use.
    pub scheme: Option<Scheme>,

    /// Optional host.
    ///
    /// Not set by default but reserved for custom use.
    pub host: Option<ByteString>,

    /// Optional port.
    ///
    /// Not set by default but reserved for custom use.
    pub port: Option<u16>,

    /// Optional media type.
    ///
    /// Not set by default but reserved for custom use.
    pub media_type: Option<MediaType>,

    /// Optional languages (sorted).
    ///
    /// Not set by default but reserved for custom use.
    pub languages: Option<BTreeSet<Language>>,

    /// Optional extensions (sorted by key).
    ///
    /// Not set by default but reserved for custom use.
    pub extensions: Option<BTreeMap<Bytes, Bytes>>,
}

impl CommonCacheKey {
    /// Constructor.
    pub fn new(
        method: Method,
        path: Option<ByteString>,
        query: Option<QueryMap>,
        scheme: Option<Scheme>,
        host: Option<ByteString>,
        port: Option<u16>,
        media_type: Option<MediaType>,
        languages: Option<BTreeSet<Language>>,
        extensions: Option<BTreeMap<Bytes, Bytes>>,
    ) -> Self {
        Self { method, scheme, host, port, path, query, media_type, languages, extensions }
    }
}

impl CacheKey for CommonCacheKey {
    fn for_request(method: &Method, uri: &Uri, _headers: &HeaderMap) -> Self {
        let (path, query) = uri
            .path_and_query()
            .map(|path_and_query| (Some(path_and_query.path().into()), path_and_query.decoded_query_map()))
            .unwrap_or_default();

        Self::new(method.clone(), path, query, None, None, None, None, None, None)
    }
}

impl CacheWeight for CommonCacheKey {
    fn cache_weight(&self) -> usize {
        const SELF_SIZE: usize = size_of::<CommonCacheKey>();

        let mut size = SELF_SIZE;

        if let Some(host) = &self.host {
            size += host.len();
        }

        if let Some(path) = &self.path {
            size += path.len();
        }

        if let Some(query) = &self.query {
            for (k, v) in query {
                size += k.len() + v.len();
            }
        }

        if let Some(media_type) = &self.media_type {
            size += media_type.cache_weight();
        }

        if let Some(languages) = &self.languages {
            for language in languages {
                size += language.cache_weight();
            }
        }

        if let Some(extensions) = &self.extensions {
            for (k, v) in extensions {
                size += k.len() + v.len();
            }
        }

        size
    }
}

impl fmt::Display for CommonCacheKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let scheme = self.scheme.as_ref().map(|scheme| scheme.as_str()).unwrap_or_default();
        let host = self.host.as_ref().map(|host| AsRef::<str>::as_ref(host)).unwrap_or_default();
        let port = self.port.map(|port| port.to_string()).unwrap_or_default();
        let path = self.path.as_ref().map(|path| AsRef::<str>::as_ref(path)).unwrap_or_default();

        let query = self
            .query
            .as_ref()
            .map(|parameter| {
                let mut string = String::default();
                for (key, values) in parameter {
                    for value in values {
                        if !string.is_empty() {
                            string += "&"
                        }
                        string += &format!("{}={}", key, value);
                    }
                }
                string
            })
            .unwrap_or_default();

        let media_type = self.media_type.as_ref().map(|media_type| media_type.to_string()).unwrap_or_default();

        let languages = self
            .languages
            .as_ref()
            .map(|languages| {
                let languages: Vec<_> = languages.iter().map(|language| language.to_string()).collect();
                languages.join(",")
            })
            .unwrap_or_default();

        let extensions = self
            .extensions
            .as_ref()
            .map(|extension| {
                let mut string = String::default();
                for (key, value) in extension {
                    if !string.is_empty() {
                        string += "&"
                    }
                    // We only display the length
                    string += &format!("{}={}", key.len(), value.len());
                }
                string
            })
            .unwrap_or_default();

        write!(
            formatter,
            "{}|{}|{}|{}|{}|{}|{}|{}|{}",
            self.method, scheme, host, port, path, query, media_type, languages, extensions
        )
    }
}
