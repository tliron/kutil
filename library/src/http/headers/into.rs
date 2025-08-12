use http::*;

//
// IntoHeaderValue
//

/// Into [HeaderValue].
///
/// (We need our own trait becase we cannot implement [Into]\<[HeaderValue]\> for external types.)
pub trait IntoHeaderValue {
    /// To [HeaderValue].
    fn into_header_value(self) -> HeaderValue;
}

impl IntoHeaderValue for HeaderValue {
    fn into_header_value(self) -> HeaderValue {
        self
    }
}
