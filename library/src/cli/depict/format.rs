use super::context::*;

/// Depiction configuration: format.
pub const DEPICTION_CONFIGURATION_FORMAT: &str = "format";

//
// DepictionFormat
//

/// Depiction format.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum DepictionFormat {
    /// A single-line depiction with minimal use of whitespace.
    ///
    /// Some optional information may be omitted.
    Compact,

    /// A combination of compact and verbose formatting.
    ///
    /// Some optional information may be omitted.
    ///
    /// This is the default format.
    #[default]
    Optimized,

    /// A consistent full depiction with all optional information.
    Verbose,
}

//
// DepictionFormatUtilities
//

/// Utilities for [DepictionFormat].
pub trait DepictionFormatUtilities {
    /// Get [DepictionFormat].
    fn get_format(&self) -> DepictionFormat;

    /// Set [DepictionFormat].
    fn with_format(self, format: DepictionFormat) -> Self;
}

impl<'own> DepictionFormatUtilities for DepictionContext<'own> {
    fn get_format(&self) -> DepictionFormat {
        match self.configuration.get(DEPICTION_CONFIGURATION_FORMAT) {
            Some(format) => match format.as_str() {
                "compact" => DepictionFormat::Compact,
                "verbose" => DepictionFormat::Verbose,
                _ => DepictionFormat::Optimized,
            },

            None => DepictionFormat::Optimized,
        }
    }

    fn with_format(self, format: DepictionFormat) -> Self {
        self.with_configuration(
            DEPICTION_CONFIGURATION_FORMAT,
            match format {
                DepictionFormat::Compact => "compact",
                DepictionFormat::Verbose => "verbose",
                DepictionFormat::Optimized => "reduced",
            },
        )
    }
}
