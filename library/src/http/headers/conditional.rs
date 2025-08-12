use super::{date::*, headers::*};

use http::*;

/// Conditional HTTP.
///
/// If there is not enough information we will assume that we have been modified and return true.
pub fn modified(request_headers: &HeaderMap, response_headers: &HeaderMap) -> bool {
    // `If-None-Match` takes precedence over `If-Modified-Since`

    // Note that ETagMatch::Any has a special meaning when not GET or HEAD
    if let Some(if_none_match) = request_headers.if_none_match()
        && if_none_match.matches(response_headers.etag().as_ref())
    {
        tracing::debug!("not modified (If-None-Match)");
        return false;
    }

    if !modified_since(response_headers.last_modified(), request_headers.if_modified_since()) {
        tracing::debug!("not modified (If-Modified-Since)");
        return false;
    }

    // Note that `If-Match` and `If-Unmodified-Since` have different uses:
    // https://stackoverflow.com/questions/2157124/http-if-none-match-vs-if-match

    true
}
