use super::{
    super::{
        super::{super::transcoding::*, headers::*},
        key::*,
    },
    configuration::*,
    hooks::*,
};

use http::*;

//
// CacheableEncodableRequest
//

/// Cacheable and/or encodable request.
pub trait CacheableEncodableRequest<RequestBodyT> {
    /// May call `cacheable_by_request` hook.
    fn should_skip_cache<CacheT, CacheKeyT>(
        &self,
        configuration: &MiddlewareCachingConfiguration<RequestBodyT, CacheT, CacheKeyT>,
    ) -> bool;

    /// May call `cache_key` hook.
    fn cache_key_with_hook<CacheT, CacheKeyT>(
        &self,
        configuration: &MiddlewareCachingConfiguration<RequestBodyT, CacheT, CacheKeyT>,
    ) -> CacheKeyT
    where
        CacheKeyT: CacheKey;

    /// May call `encodable_by_request` hook.
    fn select_encoding(&self, configuration: &MiddlewareEncodingConfiguration) -> Encoding;
}

impl<RequestBodyT> CacheableEncodableRequest<RequestBodyT> for Request<RequestBodyT> {
    fn should_skip_cache<CacheT, CacheKeyT>(
        &self,
        configuration: &MiddlewareCachingConfiguration<RequestBodyT, CacheT, CacheKeyT>,
    ) -> bool {
        let mut skip_cache = if !configuration.cache.is_none() {
            let method = self.method();
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
            && let Some(cacheable) = &configuration.cacheable_by_request
            && !cacheable(CacheableHookContext::new(self.uri(), self.headers()))
        {
            tracing::debug!("skip (cacheable_by_request=false)");
            skip_cache = true;
        }

        skip_cache
    }

    fn cache_key_with_hook<CacheT, CacheKeyT>(
        &self,
        configuration: &MiddlewareCachingConfiguration<RequestBodyT, CacheT, CacheKeyT>,
    ) -> CacheKeyT
    where
        CacheKeyT: CacheKey,
    {
        let mut cache_key = self.cache_key();

        if let Some(cache_key_hook) = &configuration.cache_key {
            cache_key_hook(CacheKeyHookContext::new(&mut cache_key, self));
        }

        cache_key
    }

    fn select_encoding(&self, configuration: &MiddlewareEncodingConfiguration) -> Encoding {
        let encoding = match &configuration.enabled_encodings_by_preference {
            Some(enabled_encodings) => {
                if !enabled_encodings.is_empty() {
                    self.headers().accept_encoding().best(enabled_encodings).cloned().unwrap_or_default().into()
                } else {
                    return Encoding::Identity;
                }
            }

            None => return Encoding::Identity,
        };

        if encoding != Encoding::Identity
            && let Some(encodable) = &configuration.encodable_by_request
            && !encodable(EncodableHookContext::new(&encoding, self.uri(), self.headers()))
        {
            tracing::debug!("not encoding to {} (encodable_by_request=false)", encoding);
            return Encoding::Identity;
        }

        encoding
    }
}
