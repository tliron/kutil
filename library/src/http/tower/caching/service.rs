use super::super::super::{
    super::std::{error::*, future::*, immutable::*},
    cache::{middleware::*, *},
    headers::*,
    transcoding::*,
};

use {
    http::{request::*, response::*},
    http_body::*,
    std::{convert::*, mem, result::Result, sync::*, task::*},
    tower::*,
};

//
// CachingService
//

/// HTTP response caching service.
///
/// See [CachingLayer](super::layer::CachingLayer).
pub struct CachingService<InnerServiceT, RequestBodyT, CacheT, CacheKeyT = CommonCacheKey>
where
    CacheT: Cache<CacheKeyT>,
    CacheKeyT: CacheKey,
{
    inner_service: InnerServiceT,
    caching: MiddlewareCachingConfiguration<RequestBodyT, CacheT, CacheKeyT>,
    encoding: MiddlewareEncodingConfiguration,
}

impl<InnerServiceT, RequestBodyT, CacheT, CacheKeyT> CachingService<InnerServiceT, RequestBodyT, CacheT, CacheKeyT>
where
    CacheT: Cache<CacheKeyT>,
    CacheKeyT: CacheKey,
{
    /// Constuctor.
    pub fn new(
        inner_service: InnerServiceT,
        caching: MiddlewareCachingConfiguration<RequestBodyT, CacheT, CacheKeyT>,
        encoding: MiddlewareEncodingConfiguration,
    ) -> Self {
        assert!(caching.inner.min_body_size <= caching.inner.max_body_size);
        Self { inner_service, caching: caching.clone(), encoding: encoding.clone() }
    }

    // Clone while keeping `inner_service`.
    //
    // See: https://docs.rs/tower/latest/tower/trait.Service.html#be-careful-when-cloning-inner-services
    fn clone_and_keep_inner_service(&mut self) -> Self
    where
        InnerServiceT: Clone,
    {
        let mut clone = self.clone();
        clone.inner_service = mem::replace(&mut self.inner_service, clone.inner_service);
        clone
    }

    // Handle request.
    async fn handle<ResponseBodyT>(
        mut self,
        request: Request<RequestBodyT>,
    ) -> Result<Response<TranscodingBody<ResponseBodyT>>, InnerServiceT::Error>
    where
        InnerServiceT: Service<Request<RequestBodyT>, Response = Response<ResponseBodyT>>,
        ResponseBodyT: 'static + Body + From<Bytes> + Send + Unpin,
        ResponseBodyT::Data: From<Bytes> + Send,
        ResponseBodyT::Error: Into<CapturedError>,
    {
        if request.should_skip_cache(&self.caching) {
            // Capture request data before moving the request to the inner service
            let uri = request.uri().clone();
            let encoding = request.select_encoding(&self.encoding);
            let content_length = request.headers().content_length();

            return self.inner_service.call(request).await.map(|upstream_response| {
                let (encoding, _skip_encoding) =
                    upstream_response.validate_encoding(&uri, encoding, content_length, &self.encoding);
                upstream_response.with_transcoding_body(&encoding, self.encoding.inner.encodable_by_default)
            });
        }

        let cache = self.caching.cache.clone().expect("has cache");
        let cache_key = request.cache_key_with_hook(&self.caching);

        match cache.get(&cache_key).await {
            Some(cached_response) => Ok({
                if modified(request.headers(), cached_response.headers()) {
                    tracing::debug!("hit");

                    cached_response
                        .to_transcoding_response(
                            &request.select_encoding(&self.encoding),
                            false,
                            cache,
                            cache_key,
                            &self.encoding.inner,
                        )
                        .await
                } else {
                    tracing::debug!("hit (not modified)");

                    not_modified_transcoding_response()
                }
            }),

            None => {
                // Capture request data before moving the request to the inner service
                let uri = request.uri().clone();
                let encoding = request.select_encoding(&self.encoding);

                let upstream_response = self.inner_service.call(request).await?;

                Ok({
                    let (skip_caching, content_length) = upstream_response.should_skip_cache(&uri, &self.caching);
                    let (encoding, skip_encoding) =
                        upstream_response.validate_encoding(&uri, encoding.clone(), content_length, &self.encoding);

                    if skip_caching {
                        upstream_response.with_transcoding_body(&encoding, self.encoding.inner.encodable_by_default)
                    } else {
                        tracing::debug!("miss");

                        match CachedResponse::new_for(
                            &uri,
                            upstream_response,
                            content_length,
                            encoding.clone(),
                            skip_encoding,
                            &self.caching.inner,
                            &self.encoding.inner,
                        )
                        .await
                        {
                            Ok(cached_response) => {
                                tracing::debug!("store ({})", encoding);
                                Arc::new(cached_response)
                                    .to_transcoding_response(&encoding, true, cache, cache_key, &self.encoding.inner)
                                    .await
                            }

                            Err(error) => match error.pieces {
                                Some(pieces) => {
                                    tracing::debug!("skip ({})", error.error);
                                    pieces.response.with_transcoding_body_with_first_bytes(
                                        Some(pieces.first_bytes),
                                        &encoding,
                                        self.encoding.inner.encodable_by_default,
                                    )
                                }

                                None => {
                                    tracing::error!("could not create cache entry: {} {}", cache_key, error);
                                    error_transcoding_response()
                                }
                            },
                        }
                    }
                })
            }
        }
    }
}

impl<InnerServiceT, RequestBodyT, CacheT, CacheKeyT> Clone
    for CachingService<InnerServiceT, RequestBodyT, CacheT, CacheKeyT>
where
    InnerServiceT: Clone,
    CacheT: Cache<CacheKeyT>,
    CacheKeyT: CacheKey,
{
    fn clone(&self) -> Self {
        Self {
            inner_service: self.inner_service.clone(),
            caching: self.caching.clone(),
            encoding: self.encoding.clone(),
        }
    }
}

impl<InnerServiceT, RequestBodyT, ResponseBodyT, ErrorT, CacheT, CacheKeyT> Service<Request<RequestBodyT>>
    for CachingService<InnerServiceT, RequestBodyT, CacheT, CacheKeyT>
where
    InnerServiceT:
        'static + Service<Request<RequestBodyT>, Response = Response<ResponseBodyT>, Error = ErrorT> + Clone + Send,
    InnerServiceT::Future: Send,
    RequestBodyT: 'static + Send,
    ResponseBodyT: 'static + Body + From<Bytes> + Send + Unpin,
    ResponseBodyT::Data: From<Bytes> + Send,
    ResponseBodyT::Error: Into<CapturedError>,
    CacheT: Cache<CacheKeyT>,
    CacheKeyT: CacheKey,
{
    type Response = Response<TranscodingBody<ResponseBodyT>>;
    type Error = InnerServiceT::Error;
    type Future = CapturedFuture<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, context: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        // Note that if we are using the cache, we technically don't have to depend on the inner
        // service being poll_ready for us to be poll_ready, however Tower's design does not allow
        // us to optimize here
        self.inner_service.poll_ready(context)
    }

    fn call(&mut self, request: Request<RequestBodyT>) -> Self::Future {
        // We unfortunately must clone the `&mut self` because it cannot be sent to the future as is;
        //
        // The worry is that we are cloning our inner service, too, which will clone *its* inner service,
        // and so on... It can be a sizeable clone if there are many service layers
        //
        // But this seems to be standard practice in Tower due to its design!

        let cloned_self = self.clone_and_keep_inner_service();
        capture_async! { cloned_self.handle(request).await }
    }
}
