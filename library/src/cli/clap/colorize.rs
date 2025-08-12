use {anstream::ColorChoice, clap::*};

//
// Colorize
//

/// Colorization options for Clap.
#[derive(Clone, Copy, Default, ValueEnum)]
pub enum Colorize {
    /// Colorize if supported.
    #[default]
    True,

    /// Don't colorize.
    False,

    /// Colorize even if not supported.
    Force,
}

impl Colorize {
    /// Applies the colorization option globally.
    pub fn initialize(&self) {
        match self {
            Self::True => {}
            Self::False => ColorChoice::Never.write_global(),
            Self::Force => ColorChoice::Always.write_global(),
        }
    }
}
