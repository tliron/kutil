use {
    http::{header::*, *},
    std::path::*,
    tower::*,
    tower_http::services::fs::*,
};

/// Create a [Response] from a file.
pub async fn response_from_file<PathT>(path: PathT, negotiable: bool) -> Response<ServeFileSystemResponseBody>
where
    PathT: AsRef<Path>,
{
    let mut response = ServeFile::new(path).oneshot(Request::new(())).await.expect("infallible");

    if !negotiable {
        response.headers_mut().remove(LAST_MODIFIED);
    }

    response
}
