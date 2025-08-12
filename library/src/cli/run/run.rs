use super::run_error::*;

use {
    anstream::eprintln,
    owo_colors::OwoColorize,
    std::{fmt, process::*},
};

//
// Runner
//

/// A replacement for `main`.
pub type Runner<ErrorT> = fn() -> Result<(), ErrorT>;

/// Runs a [Runner] and returns an [ExitCode].
///
/// Unhandled errors will be displayed in red in stderr.
pub fn run<ErrorT>(run: Runner<ErrorT>) -> ExitCode
where
    ErrorT: RunError + fmt::Display,
{
    match run() {
        Ok(_) => ExitCode::SUCCESS,

        Err(error) => {
            let (handled, code) = error.handle();

            if !handled {
                let error = error.to_string();
                if !error.is_empty() {
                    eprintln!("{}", error.red());
                }
            }

            ExitCode::from(code)
        }
    }
}
