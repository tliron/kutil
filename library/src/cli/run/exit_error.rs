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
            Some(message) => write!(formatter, "exit code {}: {}", self.code, message),
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

/// Handle an [ExitError] enum variant by implementing [RunError](super::RunError).
#[macro_export]
macro_rules! handle_exit_error {
    ( $error_enum:ty, $exit_error_variant:ident $(,)? ) => {
        impl $crate::cli::run::RunError for $error_enum {
            fn handle(&self) -> (bool, u8) {
                (
                    false,
                    match self {
                        Self::$exit_error_variant(exit_error) => exit_error.code,
                        _ => 1,
                    },
                )
            }
        }
    };
}

#[allow(unused_imports)]
pub use handle_exit_error;
