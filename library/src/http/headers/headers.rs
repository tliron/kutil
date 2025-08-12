use super::{
    super::super::std::{
        collections::*,
        immutable::{Bytes, *},
    },
    bool::*,
    encoding::*,
    etag::*,
    into::*,
    language::*,
    media_type::*,
    preferences::*,
};

use {
    http::header::*,
    httpdate::*,
    std::{any::*, fmt, str::*, time::*},
};

//
// HeaderValues
//

/// Access header values.
pub trait HeaderValues {
    // General get

    /// Parse a header value as an ASCII string.
    ///
    /// [None] could mean that there is no such header *or* that it is not a valid
    /// ASCII string.
    fn string_value(&self, name: HeaderName) -> Option<&str>;

    /// Parse all header values as ASCII strings.
    ///
    /// Will skip over non-ASCII values.
    fn string_values(&self, name: HeaderName) -> Vec<&str>;

    /// Parse a header value as an ASCII string.
    ///
    /// [None] could mean that there is no such header *or* that it is not a valid
    /// ASCII string.
    ///
    /// Unfortunately this is *not* zero-copy because [HeaderValue] does not give us access to its
    /// inner [Bytes].
    fn byte_string_value(&self, name: HeaderName) -> Option<ByteString>;

    /// Parse all header values as ASCII strings.
    ///
    /// Will skip over non-ASCII values.
    ///
    /// Unfortunately this is *not* zero-copy because [HeaderValue] does not give us access to its
    /// inner [Bytes].
    fn byte_string_values(&self, name: HeaderName) -> Vec<ByteString>;

    /// Parse a header value as a boolean ("true" or "false") or return a default a value.
    fn bool_value(&self, name: HeaderName, default: bool) -> bool {
        if let Some(value) = self.string_value(name) {
            match value.to_lowercase().as_str() {
                "true" => return true,
                "false" => return false,
                _ => {}
            }
        }

        default
    }

    /// Parse a header from its ASCII string value.
    ///
    /// [None] could mean that there is no such header *or* that it is malformed.
    fn parse_value<FromStrT>(&self, name: HeaderName) -> Option<FromStrT>
    where
        FromStrT: FromStr,
        FromStrT::Err: fmt::Display,
    {
        match self.string_value(name)?.parse() {
            Ok(value) => Some(value),

            Err(error) => {
                tracing::warn!("malformed {}: {}", type_name::<FromStrT>(), error);
                None
            }
        }
    }

    /// Parse, combine, and sort all header values as [Preferences].
    ///
    /// Will skip over malformed values.
    fn preferences<FormatT>(&self, name: HeaderName) -> Preferences<FormatT>
    where
        FormatT: Clone + Eq + FromStr,
    {
        Preferences::parse(&self.string_values(name))
    }

    /// Parse a header value as a [Duration].
    ///
    /// [None] could mean that there is no such header *or* that it is malformed.
    ///
    /// See [duration-str](https://github.com/baoyachi/duration-str).
    fn duration_value(&self, name: HeaderName) -> Option<Duration> {
        match duration_str::parse(self.string_value(name)?) {
            Ok(value) => Some(value),

            Err(error) => {
                tracing::warn!("malformed duration: {}", error);
                None
            }
        }
    }

    /// Parse a header value as an [HttpDate].
    ///
    /// [None] could mean that there is no such header *or* that it is malformed.
    fn date_value(&self, name: HeaderName) -> Option<HttpDate> {
        self.parse_value(name)
    }

    // General set

    /// Set a header value.
    ///
    /// Makes sure to remove existing values first.
    fn set_value<ValueT>(&mut self, name: HeaderName, value: ValueT)
    where
        ValueT: Into<HeaderValue>;

    /// Set a header value.
    ///
    /// Makes sure to remove existing values first.
    fn set_into_header_value<ValueT>(&mut self, name: HeaderName, value: ValueT)
    where
        ValueT: IntoHeaderValue;

    /// Set a header string value.
    ///
    /// Makes sure to remove existing values first.
    ///
    /// Will fail if not an ASCII string.
    fn set_string_value(&mut self, name: HeaderName, value: &str) -> Result<(), InvalidHeaderValue>;

    /// Set header string values.
    ///
    /// Invalid header names will be skipped.
    ///
    /// Makes sure to remove existing values first for each header.
    ///
    /// Will fail if a value is not an ASCII string.
    fn set_string_values<IteratorT, NameT, ValueT>(
        &mut self,
        name_value_pairs: IteratorT,
    ) -> Result<(), InvalidHeaderValue>
    where
        IteratorT: Iterator<Item = (NameT, ValueT)>,
        NameT: AsRef<str>,
        ValueT: AsRef<str>,
    {
        for (name, value) in name_value_pairs {
            if let Ok(name) = HeaderName::from_lowercase(name.as_ref().to_lowercase().as_bytes()) {
                self.set_string_value(name, value.as_ref())?;
            }
        }
        Ok(())
    }

    /// Set a boolean header value ("true" or "false").
    ///
    /// Makes sure to remove existing values first.
    fn set_bool_value(&mut self, name: HeaderName, value: bool) {
        self.set_into_header_value(name, if value { TRUE_HEADER_VALUE } else { FALSE_HEADER_VALUE });
    }

    // Request and response headers

    /// Parse the [`Content-Length`](CONTENT_LENGTH) header value.
    ///
    /// [None] could mean that there is no such header *or* that it is malformed.
    fn content_length(&self) -> Option<usize> {
        self.parse_value(CONTENT_LENGTH)
    }

    /// Parse the [`Content-Type`](CONTENT_TYPE) header value.
    ///
    /// [None] could mean that there is no such header *or* that it is malformed.
    fn content_type(&self) -> Option<MediaType> {
        self.parse_value(CONTENT_TYPE)
    }

    // Request headers

    /// Parse, combine, and sort all [`Accept`](ACCEPT) request header values.
    ///
    /// Will skip over malformed values.
    fn accept(&self) -> Preferences<MediaTypeSelector> {
        self.preferences(ACCEPT)
    }

    /// Parse, combine, and sort all [`Accept-Encoding`](ACCEPT_ENCODING) request header values.
    ///
    /// Will skip over malformed values.
    fn accept_encoding(&self) -> Preferences<EncodingHeaderValue> {
        self.preferences(ACCEPT_ENCODING)
    }

    /// Parse, combine, and sort all [`Accept-Language`](ACCEPT_LANGUAGE) request header values.
    ///
    /// Note that we convert all subtags to lowercase for comparison efficiency.
    ///
    /// Will skip over malformed values.
    fn accept_language(&self) -> Preferences<Language> {
        self.preferences(ACCEPT_LANGUAGE)
    }

    /// Parse the [`If-Modified-Since`](IF_MODIFIED_SINCE) request header value.
    ///
    /// [None] could mean that there is no such header *or* that it is malformed.
    fn if_modified_since(&self) -> Option<HttpDate> {
        self.date_value(IF_MODIFIED_SINCE)
    }

    /// Parse the [`If-None-Match`](IF_NONE_MATCH) request header value.
    ///
    /// [None] could mean that there is no such header *or* that it is malformed.
    fn if_none_match(&self) -> Option<ETagMatcher> {
        self.parse_value(IF_NONE_MATCH)
    }

    /// Parse the [`If-Unmodified-Since`](IF_UNMODIFIED_SINCE) request header value.
    ///
    /// [None] could mean that there is no such header *or* that it is malformed.
    fn if_unmodified_since(&self) -> Option<HttpDate> {
        self.date_value(IF_UNMODIFIED_SINCE)
    }

    /// Parse the [`If-Match`](IF_MATCH) request header value.
    ///
    /// [None] could mean that there is no such header *or* that it is malformed.
    fn if_match(&self) -> Option<ETagMatcher> {
        self.parse_value(IF_MATCH)
    }

    /// Parse the [`Authorization`](AUTHORIZATION) request header value for the `Basic` scheme.
    ///
    /// Expects UTF-8 strings.
    ///
    /// Returns the username and password.
    ///
    /// [None] could mean that there is no such header *or* that it is malformed.
    fn authorization_basic(&self) -> Option<(String, String)> {
        if let Some(authorization) = self.string_value(AUTHORIZATION)
            && authorization.starts_with("Basic ")
        {
            let authorization = &authorization[6..];
            if let Ok(authorization) = base64_simd::STANDARD.decode_to_vec(authorization)
                && let Ok(authorization) = str::from_utf8(&authorization)
                && let Some((username, password)) = authorization.split_once(':')
            {
                return Some((username.into(), password.into()));
            }
        }

        None
    }

    // Response headers

    /// Parse the [`Content-Encoding`](CONTENT_ENCODING) response header value.
    ///
    /// Defaults to [Identity](kutil_transcoding::Encoding::Identity) if there is no such header
    /// *or* that is malformed.
    fn content_encoding(&self) -> EncodingHeaderValue {
        self.parse_value(CONTENT_ENCODING).unwrap_or_default()
    }

    /// Parse the [`Content-Language`](CONTENT_LANGUAGE) response header value.
    ///
    /// Note that we convert all subtags to lowercase for comparison efficiency.
    ///
    /// Despite the header name, there can be more than one language listed. We will skip over
    /// duplicates.
    fn content_language(&self) -> Option<FastHashSet<Language>> {
        Language::parse_list(self.string_value(CONTENT_LANGUAGE)?)
    }

    /// Parse the [`Last-Modified`](LAST_MODIFIED) response header value.
    ///
    /// [None] mean that there is no such header *or* that it is malformed.
    fn last_modified(&self) -> Option<HttpDate> {
        self.date_value(LAST_MODIFIED)
    }

    /// Parse the [`ETag`](ETAG) response header value.
    ///
    /// [None] could mean that there is no such header *or* that it is malformed.
    fn etag(&self) -> Option<ETag> {
        self.parse_value(ETAG)
    }
}

impl HeaderValues for HeaderMap {
    fn string_value(&self, name: HeaderName) -> Option<&str> {
        match self.get(name)?.to_str() {
            Ok(value) => Some(value),

            Err(error) => {
                tracing::warn!("value is not ASCII: {}", error);
                None
            }
        }
    }

    fn string_values(&self, name: HeaderName) -> Vec<&str> {
        self.get_all(name).iter().filter_map(|value| value.to_str().ok()).collect()
    }

    fn byte_string_value(&self, name: HeaderName) -> Option<ByteString> {
        let bytes = Bytes::copy_from_slice(self.get(name)?.as_bytes());
        match ByteString::try_from(bytes) {
            Ok(value) => Some(value),

            Err(error) => {
                tracing::warn!("value is not ASCII: {}", error);
                None
            }
        }
    }

    fn byte_string_values(&self, name: HeaderName) -> Vec<ByteString> {
        self.get_all(name)
            .iter()
            .filter_map(|value| {
                let bytes = Bytes::copy_from_slice(value.as_bytes());
                ByteString::try_from(bytes).ok()
            })
            .collect()
    }

    fn set_value<ValueT>(&mut self, name: HeaderName, value: ValueT)
    where
        ValueT: Into<HeaderValue>,
    {
        self.remove(&name);
        self.insert(name, value.into());
    }

    fn set_into_header_value<ValueT>(&mut self, name: HeaderName, value: ValueT)
    where
        ValueT: IntoHeaderValue,
    {
        self.remove(&name);
        self.insert(name, value.into_header_value());
    }

    fn set_string_value(&mut self, name: HeaderName, value: &str) -> Result<(), InvalidHeaderValue> {
        self.remove(&name);
        self.insert(name, HeaderValue::from_str(value)?);
        Ok(())
    }
}
