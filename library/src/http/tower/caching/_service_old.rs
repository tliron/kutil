use super::{
    super::super::{cache::*, headers::*, transcoding::*},
    configuration::*,
    hooks::*,
    response::*,
};

use {
    ::bytes::*,
    http::{request::*, response::*},
    http_body::*,
    kutil_std::future::*,
    kutil_transcoding::*,
    std::{convert::*, error::Error, marker::*, result::Result, sync::*, task::*},
    tower::*,
};

//
// CachingService
//

/// HTTP response caching service.
///
/// See [CachingLayer](super::layer::CachingLayer).
#[derive(Clone)]
pub struct CachingService<InnerServiceT, CacheT, CacheKeyT = CommonCacheKey>
where
    CacheT: Cache<CacheKeyT>,
    CacheKeyT: CacheKey,
{
    inner_service: InnerServiceT,

    caching: CachingConfiguration<CacheT, CacheKeyT>,
    encoding: EncodingConfiguration,

    cache_key: PhantomData<CacheKeyT>,
}

impl<InnerServiceT, CacheT, CacheKeyT> CachingService<InnerServiceT, CacheT, CacheKeyT>
where
    CacheT: Cache<CacheKeyT>,
    CacheKeyT: CacheKey,
{
    /// Constuctor.
    pub fn new(
        inner_service: InnerServiceT,
        caching: CachingConfiguration<CacheT, CacheKeyT>,
        encoding: EncodingConfiguration,
    ) -> Self {
        assert!(caching.min_body_size <= caching.max_body_size);
        Self { inner_service, caching, encoding, cache_key: PhantomData }
    }

    /// May call `cache_key` hook.
    fn cache_key<RequestBodyT>(&self, request: &Request<RequestBodyT>) -> CacheKeyT {
        let method = request.method();
        let uri = request.uri();
        let headers = request.headers();

        let mut cache_key = CacheKeyT::for_request(method, uri, headers);

        if let Some(cache_key_hook) = &self.caching.cache_key {
            cache_key_hook(CacheKeyHookContext::new(&mut cache_key, method, uri, headers));
        }

        cache_key
    }

    /// May call `cacheable_by_request` hook.
    fn should_skip_cache<RequestBodyT>(&self, request: &Request<RequestBodyT>) -> bool {
        let mut skip_cache = if !self.caching.cache.is_none() {
            let method = request.method();
            if method.is_idempotent() {
                false
            } else {
                tracing::debug!("skip (non-idempotent {})", method);
                true
            }
        } else {
            tracing::debug!("skip (disabled)");
            true
        };

        if !skip_cache
            && let Some(cacheable) = &self.caching.cacheable_by_request
            && !cacheable(CacheableHookContext::new(request.uri(), request.headers()))
        {
            tracing::debug!("skip (cacheable_by_request=false)");
            skip_cache = true;
        }

        skip_cache
    }

    /// May call `encodable_by_request` hook.
    fn select_encoding<RequestBodyT>(&self, request: &Request<RequestBodyT>) -> Encoding {
        let mut encoding = match &self.encoding.enabled_encodings_by_preference {
            Some(enabled_encodings) => {
                if !enabled_encodings.is_empty() {
                    request.headers().accept_encoding().best(enabled_encodings).cloned().unwrap_or_default().into()
                } else {
                    Encoding::Identity
                }
            }

            None => Encoding::Identity,
        };

        if encoding != Encoding::Identity
            && let Some(encodable) = &self.encoding.encodable_by_request
            && !encodable(EncodableHookContext::new(&encoding, request.uri(), request.headers()))
        {
            tracing::debug!("not encoding to {} (encodable_by_request=false)", encoding);
            encoding = Encoding::Identity;
        }

        encoding
    }
}

impl<InnerServiceT, RequestBodyT, ResponseBodyT, ErrorT, CacheT, CacheKeyT> Service<Request<RequestBodyT>>
    for CachingService<InnerServiceT, CacheT, CacheKeyT>
where
    InnerServiceT: Service<Request<RequestBodyT>, Response = Response<ResponseBodyT>, Error = ErrorT> + Send,
    InnerServiceT::Future: 'static + Send,
    ResponseBodyT: 'static + Body + From<Bytes> + Unpin + Send,
    ResponseBodyT::Data: From<Bytes> + Send,
    ResponseBodyT::Error: Error + Send + Sync,
    CacheT: Cache<CacheKeyT>,
    CacheKeyT: CacheKey,
{
    type Response = Response<TranscodingBody<ResponseBodyT>>;
    type Error = InnerServiceT::Error;
    type Future = CapturedFuture<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, context: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        // Note that if we are using the cache, we technically don't have to depend on the inner
        // service being poll_ready for us to be poll_ready, however Tower's design does not allow
        // us to optimize this
        self.inner_service.poll_ready(context)
    }

    fn call(&mut self, request: Request<RequestBodyT>) -> Self::Future {
        if self.should_skip_cache(&request) {
            // Capture request data
            // (before moving the request to the inner service)
            let uri = request.uri().clone();
            let encoding = self.select_encoding(&request);

            // Capture future
            let inner_service_future = self.inner_service.call(request);

            // Capture self fields
            let encodable_by_default = self.encoding.encodable_by_default;
            let encodable_by_response = self.encoding.encodable_by_response.clone();

            return capture_async! {
                inner_service_future.await.map(|response| {
                    let (encoding, _) = response.validate_encoding(encoding, &uri, encodable_by_response);
                    response.with_encoding_body(&encoding, encodable_by_default)
                })
            };
        }

        let cache = self.caching.cache.clone().expect("has cache");
        let cache_key = self.cache_key(&request);

        match cache.get(&cache_key) {
            Some(cached_response) => {
                // Note that we are not calling the inner service here
                // That's the whole point of caching :)

                if modified(request.headers(), cached_response.headers()) {
                    tracing::debug!("hit");

                    // Capture request data
                    let encoding = self.select_encoding(&request);

                    // Capture self fields
                    let encodable_by_default = self.encoding.encodable_by_default;
                    let keep_identity_encoding = self.encoding.keep_identity_encoding;

                    capture_async! {
                        Ok(cached_response
                            .to_transcoding_response(
                                false,
                                cache_key,
                                cache,
                                &encoding,
                                encodable_by_default,
                                keep_identity_encoding,
                            )
                            .await)
                    }
                } else {
                    tracing::debug!("hit (not modified)");

                    capture_async! {
                        Ok(not_modified_transcoding_response())
                    }
                }
            }

            None => {
                // Capture request data
                // (before moving the request to the inner service)
                let uri = request.uri().clone();
                let encoding = self.select_encoding(&request);

                // Capture future
                let inner_service_future = self.inner_service.call(request);

                // Capture self fields
                let min_body_size = self.caching.min_body_size;
                let max_body_size = self.caching.max_body_size;
                let cacheable_by_default = self.caching.cacheable_by_default;
                let cacheable_by_response = self.caching.cacheable_by_response.clone();
                let cache_duration = self.caching.cache_duration.clone();
                let encodable_by_default = self.encoding.encodable_by_default;
                let encodable_by_response = self.encoding.encodable_by_response.clone();
                let keep_identity_encoding = self.encoding.keep_identity_encoding;

                capture_async! {
                    let response = inner_service_future.await?;

                    let (encoding, skip_encoding) =
                        response.validate_encoding(encoding.clone(), &uri, encodable_by_response);

                    let (skip_caching, content_length) = response.should_skip_cache(
                        &uri,
                        min_body_size,
                        max_body_size,
                        cacheable_by_default,
                        cacheable_by_response,
                    );

                    if skip_caching {
                        Ok(response.with_encoding_body(&encoding, encodable_by_default))
                    } else {
                        tracing::debug!("miss");

                        match CachedResponse::new_for(
                            response,
                            content_length,
                            min_body_size,
                            max_body_size,
                            skip_encoding,
                            encoding.clone(),
                            encodable_by_default,
                            keep_identity_encoding,
                            cache_duration.map(|cache_duration| (cache_duration, &uri)),
                        )
                        .await
                        {
                            Ok(cached_response) => {
                                tracing::debug!("store ({})", encoding);
                                Ok(Arc::new(cached_response)
                                    .to_transcoding_response(
                                        true,
                                        cache_key,
                                        cache,
                                        &encoding,
                                        encodable_by_default,
                                        keep_identity_encoding,
                                    )
                                    .await)
                            }

                            Err((error, pieces)) => match pieces {
                                Some(pieces) => {
                                    tracing::debug!("skip (too big or too small)");
                                    Ok(pieces.response.with_encoding_body_with_first_bytes(
                                        Some(pieces.first_bytes),
                                        &encoding,
                                        encodable_by_default,
                                    ))
                                }

                                None => {
                                    tracing::error!("could not create cache entry: {} {}", cache_key, error);
                                    Ok(error_transcoding_response())
                                }
                            },
                        }
                    }
                }
            }
        }
    }
}
