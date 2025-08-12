use super::super::super::std::error::*;

use super::into::*;

use {
    http::*,
    httpdate::*,
    std::{fs::*, io, path::*, time::*},
};

/// Current time as [HttpDate].
pub fn now() -> HttpDate {
    HttpDate::from(SystemTime::now())
}

/// File modification timestamp as HttpDate.
pub fn file_modified<PathT>(path: PathT) -> io::Result<HttpDate>
where
    PathT: AsRef<Path>,
{
    let path = path.as_ref();
    metadata(path)?.modified().map(|system_time| system_time.into()).with_path(path)
}

/// Whether we have been modified since a reference date.
///
/// If there is not enough information we will assume that we have been modified and return true.
pub fn modified_since(modified_date: Option<HttpDate>, reference_date: Option<HttpDate>) -> bool {
    if let Some(last_modified) = modified_date
        && let Some(reference_date) = reference_date
        && last_modified <= reference_date
    {
        return false;
    }

    true
}

impl IntoHeaderValue for HttpDate {
    fn into_header_value(self) -> HeaderValue {
        HeaderValue::from_str(&fmt_http_date(self.into())).expect("date in HTTP header")
    }
}
