use super::{
    super::super::{
        cache::{middleware::*, *},
        headers::*,
    },
    service::*,
};

use {
    std::{marker::*, sync::*, time::*},
    tower::*,
};

//
// CachingLayer
//

/// HTTP response caching layer with integrated encoding (compression).
///
/// Though you can rely on an external caching solution instead (e.g. a reverse proxy), there are
/// good reasons to integrate the cache directly into your application. For one, direct access
/// allows for an in-process in-memory cache, which is optimal for at least the first caching tier.
///
/// When both caching and encoding are enabled it will avoid unnecessary reencoding by storing
/// encoded versions in the cache. A cache hit will thus be able to handle HTTP content negotiation
/// (the `Accept-Encoding` header) instead of the upstream. This is an important compute
/// optimization that is impossible to achieve if encoding and caching are implemented as
/// independent layers. Far too many web servers ignore this optimization and waste compute
/// resources reencoding data that has not changed.
///
/// This layer also participates in client-side caching (conditional HTTP). A cache hit will
/// respect the client's `If-None-Match` and `If-Modified-Since` headers and return a 304 (Not
/// Modified) when appropriate, saving bandwidth as well as compute resources. If you don't set a
/// `Last-Modified` header yourself then this layer will default to the instant in which the cache
/// entry was created.
///
/// For encoding we support the web's common compression formats: Brotli, Deflate, GZip, and
/// Zstandard. We select the best encoding according to our and the client's preferences (HTTP
/// content negotiation).
///
/// The cache and cache key implementations are provided as generic type parameters. The
/// [CommonCacheKey] implementation should suffice for common use cases.
///
/// Access to the cache is `async`, though note that concurrent performance will depend on the
/// actual cache implementation, the HTTP server, and of course your async runtime.
///
/// Please check out the
/// [included examples](https://github.com/tliron/kutil/tree/main/crates/http/examples)!
///
/// Requirements
/// ============
///
/// The response body type *and* its data type must both implement
/// [From]\<[Bytes](kutil_std::immutable::Bytes)\>. (This is supported by
/// [axum](https://github.com/tokio-rs/axum).) Note that even though
/// [Tokio](https://github.com/tokio-rs/tokio) I/O types are used internally, this layer does *not*
/// require a specific async runtime.
///
/// Usage notes
/// ===========
///
/// 1. By default this layer is "opt-out" for caching and encoding. You can "punch through" this
///    behavior via custom response headers (which will be removed before sending the response
///    downstream):
///
///    * Set `XX-Cache` to "false" to skip caching.
///    * Set `XX-Encode` to "false" to skip encoding.
///
///    However, you can also configure for "opt-in", *requiring* these headers to be set to "true"
///    in order to enable the features. See [cacheable_by_default](Self::cacheable_by_default) and
///    [encodable_by_default](Self::encodable_by_default).
///
/// 2. Alternatively, you can provide [cacheable_by_request](Self::cacheable_by_request),
///    [cacheable_by_response](Self::cacheable_by_response),
///    [encodable_by_request](Self::encodable_by_request),
///    and/or [encodable_by_response](Self::encodable_by_response) hooks to control these features.
///    (If not provided they are assumed to return true.) The response hooks can be workarounds for
///    when you can't add custom headers upstream.
///
/// 3. You can explicitly set the cache duration for a response via a `XX-Cache-Duration` header.
///    Its string value is parsed using [duration-str](https://github.com/baoyachi/duration-str).
///    You can also provide a [cache_duration](Self::cache_duration) hook (the
///    `XX-Cache-Duration` header will override it). The actual effect of the duration depends on
///    the cache implementation.
///
///    ([Here](https://docs.rs/moka/latest/moka/policy/trait.Expiry.html#method.expire_after_create)
///    is the logic used for the Moka implementation.)
///
/// 4. Though this layer transparently handles HTTP content negotiation for `Accept-Encoding`, for
///    which the underlying content is the same, it cannot do so for `Accept` and
///    `Accept-Language`, for which content can differ. We do, however, provide a solution for
///    situations in which negotiation can be handled *without* the upstream response: the
///    [cache_key](Self::cache_key) hook. Here you can handle negotiation yourself and update the
///    cache key accordingly, so that different content will be cached separately. [CommonCacheKey]
///    reserves fields for media type and languages, just for this purpose.
///
///    If this impossible or too cumbersome, the alternative to content negotiation is to make
///    content selection the client's responsibility by including the content type in the URL, in
///    the path itself or as a query parameter. Web browsers often rely on JavaScript to automate
///    this for users by switching to the appropriate URL, for example adding "/en" to the path to
///    select English.
///
/// General advice
/// ==============
///
/// 1. Compressing already-compressed content is almost always a waste of compute for both the
///    server and the client. For this reason it's a good idea to explicitly skip the encoding of
///    [MIME types](https://developer.mozilla.org/en-US/docs/Web/HTTP/Guides/MIME_types/Common_types)
///    that are known to be already-compressed, such as those for audio, video, and images. You can
///    do this via the [encodable_by_response](Self::encodable_by_response) hook mentioned above.
///    (See the example.)
///
/// 2. We advise setting the `Content-Length` header on your responses whenever possible as it
///    allows this layer to check for cacheability without having to read the body, and it's
///    generally a good practice that helps many HTTP components to run optimally. That said, this
///    layer will optimize as much as it can even when `Content-Length` is not available, reading
///    only as many bytes as necessary to determine if the response is cacheable and then "pushing
///    back" those bytes (zero-copy) if it decides to skip the cache and send the response
///    downstream.
///
/// 3. Make use of client-side caching by setting the `Last-Modified` and/or `ETag` headers on your
///    responses. They are of course great without server-side caching, but this layer will respect
///    them even for cached entries, returning 304 (Not Modified) when appropriate.
///
/// 4. This caching layer does *not* own the cache, meaning that you can can insert or invalidate
///    cache entries according to application events other than user requests. Example scenarios:
///
///    1. Inserting cache entries manually can be critical for avoiding "cold cache" performance
///       degradation (as well as outright failure) for busy, resource-heavy servers. You might
///       want to initialize your cache with popular entries before opening your server to
///       requests. If your cache is distributed it might also mean syncing the cache first.
///
///    2. Invalidating cache entries manually can be critical for ensuring that clients don't
///       see out-of-date data, especially when your cache durations are long. For example, when
///       certain data is deleted from your database you can make sure to invalidate all cache
///       entries that depend on that data. To simplify this, you can the data IDs to your cache
///       keys. When invalidating, you can then enumerate all existing keys that contain the
///       relevant ID. [CommonCacheKey] reserves an `extensions` fields just for this purpose.
///
/// Request handling
/// ================
///
/// Here we'll go over the complete processing flow in detail:
///
/// 1. A request arrives. Check if it is cacheable (for now). Reasons it won't be cacheable:
///
///    * Caching is disabled for this layer
///    * The request is non-idempotent (e.g. POST)
///    * If we pass the checks above then we give the
///      [cacheable_by_request](Self::cacheable_by_request) hook a chance to skip caching.
///      If it returns false then we are non-cacheable.
///
///    If the response is non-cacheable then go to "Non-cached request handling" below.
///
/// 2. Check if we have a cached response.
///
/// 3. If we do, then:
///
///    1. Select the best encoding according to our configured preferences and the priorities
///       specified in the request's `Accept-Encoding`. If the cached response has `XX-Encode`
///       header as "false" then use Identity encoding.
///
///    2. If we have that encoding in the cache then:
///
///       1. If the client sent `If-Modified-Since` then compare with our cached `Last-Modified`,
///          and if not modified then send a 304 (Not Modified) status (conditional HTTP). END.
///
///       2. Otherwise create a response from the cache entry and send it. Note that we know its
///          size so we set `Content-Length` accordingly. END.
///
///    3. Otherwise, if we don't have the encoding in the cache then check to see if the cache
///       entry has `XX-Encode` entry as "false". If so, we will choose Identity encoding and go up
///       to step 3.2.2.
///
///    4. Find the best starting point from the encodings we already have. We select them in order
///       from cheapest to decode (Identity) to the most expensive.
///
///    5. If the starting point encoding is *not* Identity then we must first decode it. If
///       `keep_identity_encoding` is true then we will store the decoded data in the cache so that
///       we can skip this step in the future (the trade-off is taking up more room in the cache).
///
///    6. Encode the body and store it in the cache.
///
///    7. Go up to step 3.2.2.
///
/// 4. If we don't have a cached response:
///
///    1. Get the upstream response and check if it is cacheable. Reasons it won't be cacheable:
///
///       * Its status code is not "success" (200 to 299)
///       * Its `XX-Cache` header is "false"
///       * It has a `Content-Range` header (we don't cache partial responses)
///       * It has a `Content-Length` header that is lower than our configured minimum or higher
///         than our configured maximum
///       * If we pass all the checks above then we give the
///         [cacheable_by_response](Self::cacheable_by_response) hook one last chance to skip
///         caching. If it returns false then we are non-cacheable.
///
///       If the upstream response is non-cacheable then go to "Non-cached request handling" below.
///
///    2. Otherwise select the best encoding according to our configured preferences and the
///       priorities specified in the request's `Accept-Encoding`. If the upstream response has
///       `XX-Encode` header as "false" or has `Content-Length` smaller than our configured
///       minimum, then use Identity encoding.
///
///    3. If the selected encoding is not Identity then we give the
///       [encodable_by_response](Self::encodable_by_response) hook one last chance to skip
///       encoding. If it returns false we set the encoding to Identity and add the `XX-Encode`
///       header as "true" for use by step 3.1 above.
///
///    4. Read the upstream response body into a buffer. If there is no `Content-Length` header
///       then make sure to read no more than our configured maximum size.
///
///    5. If there's still more data left or the data that was read is less than our configured
///       minimum size then it means the upstream response is non-cacheable, so:
///
///       1. Push the data that we read back into the front of the upstream response body.
///
///       2. Go to "Non-cached request handling" step 4 below.
///
///    6. Otherwise store the read bytes in the cache, encoding them if necessary. We know the
///       size, so we can check if it's smaller than the configured minimum for encoding, in
///       which case we use Identity encoding. We also make sure to set the cached `Last-Modified`
///       header to the current time if the header wasn't already set. Go up to step 3.2.
///
///       Note that upstream response trailers are discarded and *not* stored in the cache. (We
///       make the assumption that trailers are only relevant to "real" responses.)
///
/// ### Non-cached request handling
///
/// 1. If the upstream response has `XX-Encode` header as "false" or has `Content-Length` smaller
///    than our configured minimum, then pass it through as is. THE END.
///
///    Note that without `Content-Length` there is no way for us to check against the minimum and
///    so we must continue.
///
/// 2. Select the best encoding according to our configured preferences and the priorities
///    specified in the request's `Accept-Encoding`.
///
/// 3. If the selected encoding is not Identity then we give the
///    [encodable_by_request](Self::encodable_by_request) and
///    [encodable_by_response](Self::encodable_by_response) hooks one last chance to skip encoding.
///    If either returns false we set the encoding to Identity.
///
/// 4. If the upstream response is already in the selected encoding then pass it through. END.
///
/// 5. Otherwise, if the upstream response is Identity, then wrap it in an encoder and send it
///    downstream. Note that we do not know the encoded size in advance so we make sure there is no
///    `Content-Length` header. END.
///
/// 6. However, if the upstream response is *not* Identity, then just pass it through as is. END.
///
///    Note that this is technically wrong and in fact there is no guarantee here that the client
///    would support the upstream response's encoding. However, we implement it this way because:
///
///    1) This is likely a rare case. If you are using this middleware then you probably don't have
///       already-encoded data coming from previous layers.
///
///    2) If you do have already-encoded data, it is reasonable to expect that the encoding was
///       selected according to the request's `Accept-Encoding`.
///
///    3) It's quite a waste of compute to decode and then reencode, which goes against the goals
///       of this middleware. (We do emit a warning in the logs.)
pub struct CachingLayer<RequestBodyT, CacheT, CacheKeyT = CommonCacheKey>
where
    CacheT: Cache<CacheKeyT>,
    CacheKeyT: CacheKey,
{
    caching: MiddlewareCachingConfiguration<RequestBodyT, CacheT, CacheKeyT>,
    encoding: MiddlewareEncodingConfiguration,
}

impl<RequestBodyT, CacheT, CacheKeyT> CachingLayer<RequestBodyT, CacheT, CacheKeyT>
where
    CacheT: Cache<CacheKeyT>,
    CacheKeyT: CacheKey,
{
    /// Enable cache.
    ///
    /// Not enabled by default.
    pub fn cache(mut self, cache: CacheT) -> Self {
        self.caching.cache = Some(cache);
        self
    }

    /// Minimum size in bytes of response bodies to cache.
    ///
    /// The default is 0.
    pub fn min_cacheable_body_size(mut self, min_cacheable_body_size: usize) -> Self {
        self.caching.inner.min_body_size = min_cacheable_body_size;
        self
    }

    /// Maximum size in bytes of response bodies to cache.
    ///
    /// The default is 1 MiB.
    pub fn max_cacheable_body_size(mut self, max_cacheable_body_size: usize) -> Self {
        self.caching.inner.max_body_size = max_cacheable_body_size;
        self
    }

    /// If a response does not specify the `XX-Cache` response header then this we will assume its
    /// value is this.
    ///
    /// The default is true.
    pub fn cacheable_by_default(mut self, cacheable_by_default: bool) -> Self {
        self.caching.inner.cacheable_by_default = cacheable_by_default;
        self
    }

    /// Provide a hook to test whether a request is cacheable.
    ///
    /// Will only be called after all internal conditions are met, giving you one last chance to
    /// prevent caching.
    ///
    /// Note that the headers are *request* headers. This hook is called before we have the
    /// upstream response.
    ///
    /// [None] by default.
    pub fn cacheable_by_request(
        mut self,
        cacheable_by_request: impl Fn(CacheableHookContext) -> bool + 'static + Send + Sync,
    ) -> Self {
        self.caching.cacheable_by_request = Some(Arc::new(Box::new(cacheable_by_request)));
        self
    }

    /// Provide a hook to test whether an upstream response is cacheable.
    ///
    /// Will only be called after all internal conditions are met, giving you one last chance to
    /// prevent caching.
    ///
    /// Note that the headers are *response* headers. This hook is called *after* we get the
    /// upstream response but *before* we read its body.
    ///
    /// [None] by default.
    pub fn cacheable_by_response(
        mut self,
        cacheable_by_response: impl Fn(CacheableHookContext) -> bool + 'static + Send + Sync,
    ) -> Self {
        self.caching.cacheable_by_response = Some(Arc::new(Box::new(cacheable_by_response)));
        self
    }

    /// [None] by default.
    pub fn cache_key(
        mut self,
        cache_key: impl Fn(CacheKeyHookContext<CacheKeyT, RequestBodyT>) + 'static + Send + Sync,
    ) -> Self {
        self.caching.cache_key = Some(Arc::new(Box::new(cache_key)));
        self
    }

    /// Provide a hook to get a response's cache duration.
    ///
    /// Will only be called if an `XX-Cache-Duration` response header is *not* provided. In other
    /// words, `XX-Cache-Duration` will always override this value.
    ///
    /// Note that the headers are *response* headers.
    ///
    /// [None] by default.
    pub fn cache_duration(
        mut self,
        cache_duration: impl Fn(CacheDurationHookContext) -> Option<Duration> + 'static + Send + Sync,
    ) -> Self {
        self.caching.inner.cache_duration = Some(Arc::new(Box::new(cache_duration)));
        self
    }

    /// Enable encodings in order from most preferred to least.
    ///
    /// Will be negotiated with the client's preferences (in its `Accept-Encoding` header) to
    /// select the best.
    ///
    /// There is no need to specify [Identity](kutil_transcoding::Encoding::Identity) as it is
    /// always enabled.
    ///
    /// The default is [ENCODINGS_BY_PREFERENCE].
    pub fn enable_encodings(mut self, enabled_encodings_by_preference: Vec<EncodingHeaderValue>) -> Self {
        self.encoding.enabled_encodings_by_preference = Some(enabled_encodings_by_preference);
        self
    }

    /// Disables encoding.
    ///
    /// The default is [ENCODINGS_BY_PREFERENCE].
    pub fn disable_encoding(mut self) -> Self {
        self.encoding.enabled_encodings_by_preference = None;
        self
    }

    /// Minimum size in bytes of response bodies to encode.
    ///
    /// Note that non-cached responses without `Content-Length` cannot be checked against this
    /// value.
    ///
    /// The default is 0.
    pub fn min_encodable_body_size(mut self, min_encodable_body_size: usize) -> Self {
        self.encoding.inner.min_body_size = min_encodable_body_size;
        self
    }

    /// If a response does not specify the `XX-Encode` response header then this we will assume its
    /// value is this.
    ///
    /// The default is true.
    pub fn encodable_by_default(mut self, encodable_by_default: bool) -> Self {
        self.encoding.inner.encodable_by_default = encodable_by_default;
        self
    }

    /// Provide a hook to test whether a request is encodable.
    ///
    /// Will only be called after all internal conditions are met, giving you one last chance to
    /// prevent encoding.
    ///
    /// Note that the headers are *request* headers. This hook is called before we have the
    /// upstream response.
    ///
    /// [None] by default.
    pub fn encodable_by_request(
        mut self,
        encodable_by_request: impl Fn(EncodableHookContext) -> bool + 'static + Send + Sync,
    ) -> Self {
        self.encoding.encodable_by_request = Some(Arc::new(Box::new(encodable_by_request)));
        self
    }

    /// Provide a hook to test whether a response is encodable.
    ///
    /// Will only be called after all internal conditions are met, giving you one last chance to
    /// prevent encoding.
    ///
    /// Note that the headers are *response* headers. This hook is called *after* we get the
    /// upstream response but *before* we read its body.
    ///
    /// [None] by default.
    pub fn encodable_by_response(
        mut self,
        encodable_by_response: impl Fn(EncodableHookContext) -> bool + 'static + Send + Sync,
    ) -> Self {
        self.encoding.encodable_by_response = Some(Arc::new(Box::new(encodable_by_response)));
        self
    }

    /// Whether to keep an [Identity](kutil_transcoding::Encoding::Identity) in the cache if it is
    /// created during reencoding.
    ///
    /// Keeping it optimizes for compute with the trade-off of taking up more room in the cache.
    ///
    /// The default is true.
    pub fn keep_identity_encoding(mut self, keep_identity_encoding: bool) -> Self {
        self.encoding.inner.keep_identity_encoding = keep_identity_encoding;
        self
    }
}

impl<RequestBodyT, CacheT, CacheKeyT> Default for CachingLayer<RequestBodyT, CacheT, CacheKeyT>
where
    CacheT: Cache<CacheKeyT>,
    CacheKeyT: CacheKey,
{
    fn default() -> Self {
        Self { caching: Default::default(), encoding: Default::default() }
    }
}

impl<RequestBodyT, CacheT, CacheKeyT> Clone for CachingLayer<RequestBodyT, CacheT, CacheKeyT>
where
    CacheT: Cache<CacheKeyT>,
    CacheKeyT: CacheKey,
{
    fn clone(&self) -> Self {
        Self { caching: self.caching.clone(), encoding: self.encoding.clone() }
    }
}

impl<InnerServiceT, RequestBodyT, CacheT, CacheKeyT> Layer<InnerServiceT>
    for CachingLayer<RequestBodyT, CacheT, CacheKeyT>
where
    CacheT: Cache<CacheKeyT>,
    CacheKeyT: CacheKey,
{
    type Service = CachingService<InnerServiceT, RequestBodyT, CacheT, CacheKeyT>;

    fn layer(&self, inner_service: InnerServiceT) -> Self::Service {
        CachingService::new(inner_service, self.caching.clone(), self.encoding.clone())
    }
}
