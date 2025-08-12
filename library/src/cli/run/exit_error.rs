use std::{error, fmt};

//
// ExitError
//

/// Simple exit error.
#[derive(Clone, Debug)]
pub struct ExitError {
    /// Exit code.
    pub code: u8,

    /// Optional goodbye message.
    pub message: Option<String>,
}

impl ExitError {
    /// Constructor.
    pub fn new(code: u8, message: Option<String>) -> Self {
        Self { code, message }
    }

    /// Constructor.
    pub fn new_from<ToStringT>(code: u8, to_string: ToStringT) -> Self
    where
        ToStringT: ToString,
    {
        Self::new(code, Some(to_string.to_string()))
    }

    /// Successful exit (code 0) without a message.
    pub fn success() -> Self {
        0.into()
    }
}

impl error::Error for ExitError {}

impl fmt::Display for ExitError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.message {
            Some(message) => write!(formatter, "{}: {}", self.code, message),
            None => Ok(()),
        }
    }
}

// Conversions

impl From<u8> for ExitError {
    fn from(value: u8) -> Self {
        Self::new(value, None)
    }
}

impl From<String> for ExitError {
    fn from(message: String) -> Self {
        Self::new(1, Some(message))
    }
}

impl From<&str> for ExitError {
    fn from(message: &str) -> Self {
        message.to_string().into()
    }
}
