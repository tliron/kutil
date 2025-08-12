use super::{
    super::{
        super::{
            std::{error::*, immutable::*},
            transcoding::*,
        },
        headers::*,
    },
    body::*,
};

use {
    http::{header::*, *},
    http_body::*,
};

//
// IntoTranscodingResponse
//

/// Into a [Response] with a [TranscodingBody].
pub trait IntoTranscodingResponse<BodyT>
where
    Self: Sized,
    BodyT: Body,
    BodyT::Error: Into<CapturedError>,
{
    /// Into a [Response] with a passthrough [TranscodingBody].
    fn with_transcoding_body_passthrough(self) -> Response<TranscodingBody<BodyT>> {
        self.with_transcoding_body_passthrough_with_first_bytes(None)
    }

    /// Into a [Response] with a passthrough [TranscodingBody].
    fn with_transcoding_body_passthrough_with_first_bytes(
        self,
        first_bytes: Option<Bytes>,
    ) -> Response<TranscodingBody<BodyT>>;

    /// Into a [Response] with an encoding [TranscodingBody].
    fn with_transcoding_body(
        self,
        encoding: &Encoding,
        encodable_by_default: bool,
    ) -> Response<TranscodingBody<BodyT>> {
        self.with_transcoding_body_with_first_bytes(None, encoding, encodable_by_default)
    }

    /// Into a [Response] with an encoding [TranscodingBody].
    fn with_transcoding_body_with_first_bytes(
        self,
        first_bytes: Option<Bytes>,
        encoding: &Encoding,
        encodable_by_default: bool,
    ) -> Response<TranscodingBody<BodyT>>;
}

impl<BodyT> IntoTranscodingResponse<BodyT> for Response<BodyT>
where
    BodyT: Body,
    BodyT::Error: Into<CapturedError>,
{
    fn with_transcoding_body_passthrough_with_first_bytes(
        self,
        first_bytes: Option<Bytes>,
    ) -> Response<TranscodingBody<BodyT>> {
        let (mut parts, body) = self.into_parts();
        parts.headers.remove(XX_ENCODE);
        parts.headers.remove(XX_CACHE);
        Response::from_parts(parts, body.into_transcoding_passthrough_with_first_bytes(first_bytes))
    }

    fn with_transcoding_body_with_first_bytes(
        self,
        first_bytes: Option<Bytes>,
        encoding: &Encoding,
        encodable_by_default: bool,
    ) -> Response<TranscodingBody<BodyT>> {
        if *encoding == Encoding::Identity {
            return self.with_transcoding_body_passthrough_with_first_bytes(first_bytes);
        }

        let (mut parts, body) = self.into_parts();

        let encode = parts.headers.xx_encode(encodable_by_default);
        parts.headers.remove(XX_CACHE);
        parts.headers.remove(XX_ENCODE);

        if !encode {
            tracing::debug!("not encoding to {} ({}=false)", encoding, XX_ENCODE);
            return Response::from_parts(parts, body.into_transcoding_passthrough_with_first_bytes(first_bytes));
        }

        let current_encoding = parts.headers.content_encoding().into();

        if *encoding == current_encoding {
            tracing::debug!("already encoded as {}", encoding);
            return Response::from_parts(parts, body.into_transcoding_passthrough_with_first_bytes(first_bytes));
        }

        if current_encoding != Encoding::Identity {
            tracing::debug!("not reencoding from {} to {})", current_encoding, encoding);
            return Response::from_parts(parts, body.into_transcoding_passthrough_with_first_bytes(first_bytes));

            // We intentionally don't reencode because it would be computationally wasteful!
            // Also, it would be hard to program this in our current generics-based design.
            // The wrapping would have to look something like this:
            //
            //   BodyReader ->
            //     decoding TranscodingReader ->
            //       encoding TranscodingReader
            //
            // Also note that we are *not* checking that the client can accept current_encoding.
            // We just have to trust that the body was generated with respect to the request's
            // `Accept-Encoding`.
        }

        parts.headers.set_into_header_value(CONTENT_ENCODING, encoding.clone());

        // We don't know what the final content length will be
        parts.headers.remove(CONTENT_LENGTH);

        // We don't know what the final digest will be
        parts.headers.remove(CONTENT_DIGEST);

        Response::from_parts(parts, body.into_encoding_with_first_bytes(first_bytes, encoding))
    }
}

/// [Response] with an empty [TranscodingBody] and [StatusCode::INTERNAL_SERVER_ERROR].
pub fn error_transcoding_response<BodyT>() -> Response<TranscodingBody<BodyT>>
where
    BodyT: Body + From<Bytes>,
    BodyT::Error: Into<CapturedError>,
{
    let mut response = Response::new(Bytes::default().into()).with_transcoding_body_passthrough_with_first_bytes(None);
    *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
    response
}

/// [Response] with an empty [TranscodingBody] and [StatusCode::NOT_MODIFIED].
pub fn not_modified_transcoding_response<BodyT>() -> Response<TranscodingBody<BodyT>>
where
    BodyT: Body + From<Bytes>,
    BodyT::Error: Into<CapturedError>,
{
    let mut response = Response::new(Bytes::default().into()).with_transcoding_body_passthrough_with_first_bytes(None);
    *response.status_mut() = StatusCode::NOT_MODIFIED;
    response
}
