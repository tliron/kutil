use super::super::{super::std::immutable::*, errors::*};

use {
    http::{header::*, response::*, *},
    std::result::Result,
};

/// Creates a response with a [Bytes] body.
///
/// The `Content-Type` and `Content-Length` headers will be set, overriding existing values.
///
/// The response body must implement [From]\<[Bytes]\>.
pub fn response_from_bytes<BodyT>(
    body: Bytes,
    content_type: &str,
    headers: HeaderMap,
) -> Result<Response<BodyT>, StatusCode>
where
    BodyT: From<Bytes>,
{
    let mut builder = Response::builder();

    for (name, value) in headers.iter() {
        match *name {
            CONTENT_TYPE | CONTENT_LENGTH => {}
            _ => builder = builder.header(name, value),
        };
    }

    builder
        .header(CONTENT_TYPE, content_type)
        .header(CONTENT_LENGTH, body.len())
        .body(body.into())
        .map_err_internal_server("build response")
}
