use super::super::super::super::{
    super::{
        std::{error::*, immutable::*},
        transcoding::*,
    },
    cache::*,
    transcoding::*,
};

use {http::*, http_body::*};

//
// ToTranscodingResponse
//

/// To transcoding response.
#[allow(async_fn_in_trait)]
pub trait ToTranscodingResponse {
    /// To a [Response] with a [TranscodingBody].
    ///
    /// Will update the cache if we are modified.
    ///
    /// If we encounter an error will return a response with [StatusCode::INTERNAL_SERVER_ERROR].
    async fn to_transcoding_response<ResponseBodyT, CacheT, CacheKeyT>(
        self,
        encoding: &Encoding,
        is_new: bool,
        cache: CacheT,
        key: CacheKeyT,
        configuration: &EncodingConfiguration,
    ) -> Response<TranscodingBody<ResponseBodyT>>
    where
        ResponseBodyT: 'static + Body + From<Bytes> + Send + Unpin,
        ResponseBodyT::Data: From<Bytes> + Send,
        ResponseBodyT::Error: Into<CapturedError>,
        CacheT: Cache<CacheKeyT>,
        CacheKeyT: CacheKey;
}

impl ToTranscodingResponse for CachedResponseRef {
    /// To a [Response] with a [TranscodingBody].
    ///
    /// Will update the cache if we are modified.
    ///
    /// If we encounter an error will return a response with [StatusCode::INTERNAL_SERVER_ERROR].
    async fn to_transcoding_response<ResponseBodyT, CacheT, CacheKeyT>(
        self,
        encoding: &Encoding,
        is_new: bool,
        cache: CacheT,
        key: CacheKeyT,
        configuration: &EncodingConfiguration,
    ) -> Response<TranscodingBody<ResponseBodyT>>
    where
        ResponseBodyT: 'static + Body + From<Bytes> + Send + Unpin,
        ResponseBodyT::Data: From<Bytes>,
        ResponseBodyT::Error: Into<CapturedError>,
        CacheT: Cache<CacheKeyT>,
        CacheKeyT: CacheKey,
    {
        match self.to_response(&encoding, configuration).await {
            Ok((response, modified)) => {
                if is_new {
                    cache.put(key, self).await;
                } else if let Some(modified) = modified {
                    // A new CachedResponse should already contain our encoding
                    // and thus never cause modification!
                    assert!(!is_new);

                    cache.put(key, modified.into()).await;
                }

                response
            }

            Err(error) => {
                tracing::error!("could not create response from cache: {} {}", key, error);
                error_transcoding_response()
            }
        }
    }
}
