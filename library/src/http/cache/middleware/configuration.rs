use super::{
    super::super::{cache::*, headers::*},
    hooks::*,
};

/// Encodings in order from most preferred to least.
///
/// We are generally preferring to optimize for compute rather than bandwidth.
///
/// GZip and Deflate are almost identical, but we prefer GZip because it allows clients to check
/// for errors.
pub const ENCODINGS_BY_PREFERENCE: &[EncodingHeaderValue] = &[
    EncodingHeaderValue::Brotli,
    EncodingHeaderValue::GZip,
    EncodingHeaderValue::Deflate,
    EncodingHeaderValue::Zstandard,
];

//
// MiddlewareCachingConfiguration
//

/// Middleware caching configuration.
pub struct MiddlewareCachingConfiguration<RequestBodyT, CacheT, CacheKeyT> {
    /// Cache.
    pub cache: Option<CacheT>,

    /// Cacheable by request (hook).
    pub cacheable_by_request: Option<CacheableHook>,

    /// Cacheable by response (hook).
    pub cacheable_by_response: Option<CacheableHook>,

    /// Cache key (hook).
    pub cache_key: Option<CacheKeyHook<CacheKeyT, RequestBodyT>>,

    /// Inner configuration.
    pub inner: CachingConfiguration,
}

impl<RequestBodyT, CacheT, CacheKeyT> Default for MiddlewareCachingConfiguration<RequestBodyT, CacheT, CacheKeyT> {
    fn default() -> Self {
        Self {
            cache: None,
            cacheable_by_request: None,
            cacheable_by_response: None,
            cache_key: None,
            inner: CachingConfiguration {
                min_body_size: 0,
                max_body_size: 1024 * 1024, // 1 MiB
                cacheable_by_default: true,
                cache_duration: None,
            },
        }
    }
}

impl<RequestBodyT, CacheT, CacheKeyT> Clone for MiddlewareCachingConfiguration<RequestBodyT, CacheT, CacheKeyT>
where
    CacheT: Clone,
{
    fn clone(&self) -> Self {
        // Unfortunately we can't get away with #[derive(Clone)]
        // The culprit is the RequestBodyT generic param in CacheKeyHookContext
        Self {
            cache: self.cache.clone(),
            cacheable_by_request: self.cacheable_by_request.clone(),
            cacheable_by_response: self.cacheable_by_response.clone(),
            cache_key: self.cache_key.clone(),
            inner: self.inner.clone(),
        }
    }
}

//
// MiddlewareEncodingConfiguration
//

/// Middleware encoding configuration.
#[derive(Clone)]
pub struct MiddlewareEncodingConfiguration {
    /// Enabled encodings in order of preference.
    pub enabled_encodings_by_preference: Option<Vec<EncodingHeaderValue>>,

    /// Encodable by request (hook).
    pub encodable_by_request: Option<EncodableHook>,

    /// Encodable by response (hook).
    pub encodable_by_response: Option<EncodableHook>,

    /// Inner configuration.
    pub inner: EncodingConfiguration,
}

impl Default for MiddlewareEncodingConfiguration {
    fn default() -> Self {
        Self {
            enabled_encodings_by_preference: Some(ENCODINGS_BY_PREFERENCE.into()),
            encodable_by_request: None,
            encodable_by_response: None,
            inner: EncodingConfiguration { min_body_size: 0, encodable_by_default: true, keep_identity_encoding: true },
        }
    }
}
