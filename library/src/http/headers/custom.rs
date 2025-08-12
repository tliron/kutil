use super::headers::*;

use {http::*, std::time::*};

/// `XX-Cache` HTTP response header specifying whether to cache the response.
pub const XX_CACHE: HeaderName = HeaderName::from_static("xx-cache");

/// `XX-Cache-Duration` HTTP response header specifying the cache duration in seconds.
pub const XX_CACHE_DURATION: HeaderName = HeaderName::from_static("xx-cache-duration");

/// `XX-Encode` HTTP response header specifying whether to encode the response.
pub const XX_ENCODE: HeaderName = HeaderName::from_static("xx-encode");

/// `Content-Digest` HTTP response header.
///
/// See [MDN](https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Content-Digest).
///
/// Non-standard but ... just in case.
pub const CONTENT_DIGEST: HeaderName = HeaderName::from_static("content-digest");

//
// CustomHeaderValues
//

/// Access custom header values.
pub trait CustomHeaderValues {
    /// Parse `XX-Cache` response header value.
    fn xx_cache(&self, default: bool) -> bool;

    /// Parse `XX-Cache-Duration` response header value.
    fn xx_cache_duration(&self) -> Option<Duration>;

    /// Parse `XX-Encode` response header value.
    fn xx_encode(&self, default: bool) -> bool;
}

impl CustomHeaderValues for HeaderMap {
    fn xx_cache(&self, default: bool) -> bool {
        self.bool_value(XX_CACHE, default)
    }

    fn xx_cache_duration(&self) -> Option<Duration> {
        self.duration_value(XX_CACHE_DURATION)
    }

    fn xx_encode(&self, default: bool) -> bool {
        self.bool_value(XX_ENCODE, default)
    }
}
