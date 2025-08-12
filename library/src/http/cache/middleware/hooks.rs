use super::super::super::super::transcoding::*;

use {http::request::*, http::*, std::sync::*};

/// Hook to check if a request or a response is cacheable.
pub type CacheableHook = Arc<Box<dyn Fn(CacheableHookContext) -> bool + Send + Sync>>;

/// Hook to check if a request or a response is encodable.
pub type EncodableHook = Arc<Box<dyn Fn(EncodableHookContext) -> bool + Send + Sync>>;

/// Hook to update a request's cache key.
pub type CacheKeyHook<CacheKeyT, RequestBodyT> =
    Arc<Box<dyn Fn(CacheKeyHookContext<CacheKeyT, RequestBodyT>) + Send + Sync>>;

//
// CacheableHookContext
//

/// Context for [CacheableHook].
#[derive(Clone, Debug)]
pub struct CacheableHookContext<'own> {
    /// URI.
    pub uri: &'own Uri,

    /// Headers.
    pub headers: &'own HeaderMap,
}

impl<'own> CacheableHookContext<'own> {
    /// Constructor.
    pub fn new(uri: &'own Uri, headers: &'own HeaderMap) -> Self {
        Self { uri, headers }
    }
}

//
// EncodableHookContext
//

/// Context for [EncodableHook].
#[derive(Clone, Debug)]
pub struct EncodableHookContext<'own> {
    /// Encoding.
    pub encoding: &'own Encoding,

    /// URI.
    pub uri: &'own Uri,

    /// Headers.
    pub headers: &'own HeaderMap,
}

impl<'own> EncodableHookContext<'own> {
    /// Constructor.
    pub fn new(encoding: &'own Encoding, uri: &'own Uri, headers: &'own HeaderMap) -> Self {
        Self { encoding, uri, headers }
    }
}

//
// CacheKeyHookContext
//

/// Context for [CacheKeyHook].
#[derive(Debug)]
pub struct CacheKeyHookContext<'own, CacheKeyT, RequestBodyT> {
    /// Cache key.
    pub cache_key: &'own mut CacheKeyT,

    /// Request.
    pub request: &'own Request<RequestBodyT>,
}

impl<'own, CacheKeyT, RequestBodyT> CacheKeyHookContext<'own, CacheKeyT, RequestBodyT> {
    /// Constructor.
    pub fn new(cache_key: &'own mut CacheKeyT, request: &'own Request<RequestBodyT>) -> Self {
        Self { cache_key, request }
    }
}
