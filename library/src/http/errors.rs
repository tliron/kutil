use {
    http::*,
    std::{fmt, result::Result},
};

//
// MapErrorStatusCode
//

/// Map [Result] error to a [StatusCode].
pub trait MapErrorStatusCode<OkT> {
    /// Map [Result] error to a [StatusCode].
    fn map_err_status_code(self, status: StatusCode, message: &str) -> Result<OkT, StatusCode>;

    /// Map [Result] error to [StatusCode::INTERNAL_SERVER_ERROR].
    fn map_err_internal_server(self, message: &str) -> Result<OkT, StatusCode>;
}

impl<OkT, FromErrorT> MapErrorStatusCode<OkT> for Result<OkT, FromErrorT>
where
    FromErrorT: fmt::Display,
{
    fn map_err_status_code(self, status: StatusCode, message: &str) -> Result<OkT, StatusCode> {
        self.map_err(|error| {
            tracing::error!("{}: {}", message, error);
            status
        })
    }

    fn map_err_internal_server(self, message: &str) -> Result<OkT, StatusCode> {
        self.map_err_status_code(StatusCode::INTERNAL_SERVER_ERROR, message)
    }
}
