use super::{
    super::{
        super::{
            std::{error::*, immutable::*},
            transcoding::*,
        },
        body::*,
        headers::*,
        pieces::*,
    },
    body::*,
    configuration::*,
    hooks::*,
    weight::*,
};

use {
    core::any::*,
    duration_str::*,
    http::{header::*, response::*, *},
    http_body::*,
    std::{io, mem::*, result::Result, sync::*, time::*},
};

/// Common reference type for [CachedResponse].
pub type CachedResponseRef = Arc<CachedResponse>;

//
// CachedResponse
//

/// Cached HTTP response.
///
/// Caching the body is handled by [CachedBody].
#[derive(Clone, Debug)]
pub struct CachedResponse {
    /// Response parts.
    pub parts: Parts,

    /// Response body.
    pub body: CachedBody,

    /// Optional duration.
    pub duration: Option<Duration>,
}

impl CachedResponse {
    /// Constructor.
    ///
    /// Reads the response body and stores it as [Bytes].
    ///
    /// If `known_body_size` is not [None] then that's the size we expect. Otherwise
    /// we'll try to read to `max_body_size` and will expect at least `min_body_size`.
    ///
    /// In either case we will return an error if the body wasn't completely read (we won't cache
    /// incomplete bodies!), together with [ResponsePieces], which can be used by the caller to
    /// reconstruct the original response.
    ///
    /// `preferred_encoding` is the encoding in which we *want* to store the body. If the response's
    /// encoding is different from what we want then it will be reencoded, unless the `XX-Encode`
    /// header is "false", in which case it's as if `preferred_encoding` were
    /// [Identity](Encoding::Identity).
    ///
    /// If an [Identity](Encoding::Identity) is created during this reencoding then it will also be
    /// stored if `keep_identity_encoding` is true.
    ///
    /// If the response doesn't already have a `Last-Modified` header, we will set it to the
    /// current time.
    pub async fn new_for<BodyT>(
        uri: &Uri,
        response: Response<BodyT>,
        declared_body_size: Option<usize>,
        mut preferred_encoding: Encoding,
        skip_encoding: bool,
        caching_configuration: &CachingConfiguration,
        encoding_configuration: &EncodingConfiguration,
    ) -> Result<Self, ErrorWithResponsePieces<ReadBodyError, BodyT>>
    where
        BodyT: Body + Unpin,
        BodyT::Error: Into<CapturedError>,
    {
        let (mut parts, body) = response.into_parts();

        let bytes = match body
            .read_into_bytes_or_pieces(
                declared_body_size,
                caching_configuration.min_body_size,
                caching_configuration.max_body_size,
            )
            .await
        {
            Ok((bytes, _trailers)) => bytes,
            Err(error) => {
                return Err(ErrorWithResponsePieces::new_from_body(error, parts));
            }
        };

        if preferred_encoding != Encoding::Identity {
            if !parts.headers.xx_encode(encoding_configuration.encodable_by_default) {
                tracing::debug!("not encoding to {} ({}=false)", preferred_encoding, XX_ENCODE);
                preferred_encoding = Encoding::Identity;
            } else if bytes.len() < encoding_configuration.min_body_size {
                tracing::debug!("not encoding to {} (too small)", preferred_encoding);
                preferred_encoding = Encoding::Identity;
            }
        }

        let body = CachedBody::new_with(
            bytes,
            parts.headers.content_encoding().into(),
            preferred_encoding,
            encoding_configuration,
        )
        .await
        // This is not *exactly* a ReadBodyError, but rather an encoding error for the read body
        .map_err(|error| ErrorWithResponsePieces::from(ReadBodyError::from(error)))?;

        // Extract `XX-Cache-Duration` or call hook
        let duration = match parts.headers.xx_cache_duration() {
            Some(duration) => Some(duration),
            None => caching_configuration
                .cache_duration
                .as_ref()
                .and_then(|duration| duration(CacheDurationHookContext::new(uri, &parts.headers))),
        };

        if let Some(duration) = duration {
            tracing::debug!("duration: {}", duration.human_format());
        }

        // Make sure we have a `Last-Modified`
        if !parts.headers.contains_key(LAST_MODIFIED) {
            parts.headers.set_into_header_value(LAST_MODIFIED, now());
        }

        parts.headers.remove(XX_CACHE);
        parts.headers.remove(XX_CACHE_DURATION);
        parts.headers.remove(CONTENT_ENCODING);
        parts.headers.remove(CONTENT_LENGTH);
        parts.headers.remove(CONTENT_DIGEST);

        // Note that we are keeping the `XX-Encode` header in the cache
        // (but will remove it in `to_response`)

        if skip_encoding {
            parts.headers.set_bool_value(XX_ENCODE, true);
        }

        // TODO: can we support ranges? if so, we should not remove this header
        // https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Accept-Ranges
        parts.headers.remove(ACCEPT_RANGES);

        Ok(Self { parts, body, duration })
    }

    /// Clone with new body.
    pub fn clone_with_body(&self, body: CachedBody) -> Self {
        Self { parts: self.parts.clone(), body, duration: self.duration.clone() }
    }

    /// Headers.
    pub fn headers(&self) -> &HeaderMap {
        &self.parts.headers
    }

    /// Create a [Response].
    ///
    /// If we don't have the specified encoding then we will reencode from another encoding,
    /// storing the result so that we won't have to encode it again.
    ///
    /// If an [Identity](Encoding::Identity) is created during this reencoding then it will also be
    /// stored if `keep_identity_encoding` is true.
    ///
    /// If the stored `XX-Encode` header is "false" then will ignore the specified encoding and
    /// return an [Identity](Encoding::Identity) response.
    ///
    /// Returns a modified clone if reencoding caused a new encoding to be stored. Note that
    /// cloning should be cheap due to our use of [Bytes] in the body.
    pub async fn to_response<BodyT>(
        &self,
        mut encoding: &Encoding,
        configuration: &EncodingConfiguration,
    ) -> io::Result<(Response<BodyT>, Option<Self>)>
    where
        BodyT: Body + From<Bytes>,
    {
        if (*encoding != Encoding::Identity) && !self.headers().xx_encode(configuration.encodable_by_default) {
            tracing::debug!("not encoding to {} ({}=false)", encoding, XX_ENCODE);
            encoding = &Encoding::Identity;
        }

        let (bytes, modified) = self.body.get(encoding, configuration).await?;

        let mut parts = self.parts.clone();

        parts.headers.remove(XX_ENCODE);

        if *encoding != Encoding::Identity {
            // No need to specify Identity as it's the default
            parts.headers.set_into_header_value(CONTENT_ENCODING, encoding.clone());
        }

        parts.headers.set_value(CONTENT_LENGTH, bytes.len());

        Ok((Response::from_parts(parts, bytes.into()), modified.map(|body| self.clone_with_body(body))))
    }
}

impl CacheWeight for CachedResponse {
    fn cache_weight(&self) -> usize {
        const SELF_SIZE: usize = size_of::<CachedResponse>();
        const HEADER_MAP_ENTRY_SIZE: usize = size_of::<HeaderName>() + size_of::<HeaderValue>();
        const EXTENSION_ENTRY_SIZE: usize = size_of::<TypeId>();

        let mut size = SELF_SIZE;

        let parts = &self.parts;
        for (name, value) in &parts.headers {
            size += HEADER_MAP_ENTRY_SIZE + name.as_str().len() + value.len()
        }
        size += parts.extensions.len() * EXTENSION_ENTRY_SIZE;

        size += self.body.cache_weight();

        size
    }
}
