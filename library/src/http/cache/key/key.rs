use super::super::weight::*;

use {
    http::{header::*, uri::*, *},
    std::{fmt, hash::*},
};

//
// CacheKey
//

/// Cache key.
pub trait CacheKey
where
    Self: 'static + Clone + fmt::Display + Eq + Hash + Send + CacheWeight + Sync,
{
    /// Create a cache key for a request.
    fn for_request(method: &Method, uri: &Uri, headers: &HeaderMap) -> Self;
}

//
// CacheKeyForRequest
//

/// [CacheKey] for [Request].
pub trait CacheKeyForRequest<CacheKeyT> {
    /// Create a cache key.
    fn cache_key(&self) -> CacheKeyT;
}

impl<RequestBodyT, CacheKeyT> CacheKeyForRequest<CacheKeyT> for Request<RequestBodyT>
where
    CacheKeyT: CacheKey,
{
    fn cache_key(&self) -> CacheKeyT {
        CacheKeyT::for_request(self.method(), self.uri(), self.headers())
    }
}
