use super::super::{
    super::super::{super::transcoding::*, headers::*},
    configuration::*,
    hooks::*,
};

use http::{header::*, *};

//
// UpstreamResponse
//

/// Upstream response.
pub trait UpstreamResponse<ResponseBodyT> {
    /// Check if we should skip the cache.
    ///
    /// Also returns the value of `Content-Length` if available.
    ///
    /// If the response passes all our checks then we turn to the hook to give it one last chance
    /// to skip the cache.
    fn should_skip_cache<RequestBodyT, CacheT, CacheKeyT>(
        &self,
        uri: &Uri,
        configuration: &MiddlewareCachingConfiguration<CacheT, CacheKeyT, RequestBodyT>,
    ) -> (bool, Option<usize>);

    /// Validate encoding.
    ///
    /// Checks `content_length`, if provided, against `min_body_size`. And gives the hook one last
    /// chance to skip encoding.
    ///
    /// Will return true if we are forcing a skip.
    fn validate_encoding(
        &self,
        uri: &Uri,
        encoding: Encoding,
        content_length: Option<usize>,
        configuration: &MiddlewareEncodingConfiguration,
    ) -> (Encoding, bool);
}

impl<ResponseBodyT> UpstreamResponse<ResponseBodyT> for Response<ResponseBodyT> {
    fn should_skip_cache<RequestBodyT, CacheT, CacheKeyT>(
        &self,
        uri: &Uri,
        configuration: &MiddlewareCachingConfiguration<CacheT, CacheKeyT, RequestBodyT>,
    ) -> (bool, Option<usize>) {
        let headers = self.headers();
        let status = self.status();

        let mut skip_cache = if !headers.xx_cache(configuration.inner.cacheable_by_default) {
            tracing::debug!("skip ({}=false)", XX_CACHE);
            (true, None)
        } else if !status.is_success() {
            tracing::debug!("skip (status={})", status.as_u16());
            (true, None)
        } else if headers.contains_key(CONTENT_RANGE) {
            tracing::debug!("skip (range)");
            (true, None)
        } else {
            match headers.content_length() {
                Some(content_length) => {
                    if content_length < configuration.inner.min_body_size {
                        tracing::debug!("skip (Content-Length too small)");
                        (true, Some(content_length))
                    } else if content_length > configuration.inner.max_body_size {
                        tracing::debug!("skip (Content-Length too big)");
                        (true, Some(content_length))
                    } else {
                        (false, Some(content_length))
                    }
                }

                None => (false, None),
            }
        };

        if !skip_cache.0
            && let Some(cacheable) = &configuration.cacheable_by_response
            && !cacheable(CacheableHookContext::new(uri, headers))
        {
            tracing::debug!("skip (cacheable_by_response=false)");
            skip_cache.0 = true;
        }

        skip_cache
    }

    fn validate_encoding(
        &self,
        uri: &Uri,
        encoding: Encoding,
        content_length: Option<usize>,
        configuration: &MiddlewareEncodingConfiguration,
    ) -> (Encoding, bool) {
        if encoding == Encoding::Identity {
            (encoding, false)
        } else {
            if let Some(content_length) = content_length {
                let min_body_size = configuration.inner.min_body_size;
                if min_body_size != 0 {
                    if content_length < min_body_size {
                        tracing::debug!("not encoding to {} (too small)", encoding);
                        return (Encoding::Identity, true);
                    }
                }
            }

            match &configuration.encodable_by_response {
                Some(encodable) => {
                    if encodable(EncodableHookContext::new(&encoding, uri, self.headers())) {
                        (encoding, false)
                    } else {
                        tracing::debug!("not encoding to {} (encodable_by_response=false)", encoding);
                        (Encoding::Identity, true)
                    }
                }

                None => (encoding, false),
            }
        }
    }
}
