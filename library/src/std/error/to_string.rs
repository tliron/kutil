//
// StringResult
//

/// Map [Err] into string.
pub trait StringResult<OkT> {
    /// Map [Err] into string.
    fn into_string(self) -> Result<OkT, String>;
}

impl<OkT, ErrorT> StringResult<OkT> for Result<OkT, ErrorT>
where
    ErrorT: ToString,
{
    fn into_string(self) -> Result<OkT, String> {
        self.map_err(|error| error.to_string())
    }
}
