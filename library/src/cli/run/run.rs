use {anstream::eprintln, owo_colors::*, problemo::*, std::process::*};

//
// Runner
//

/// A replacement for `main`.
pub type Runner = fn() -> Result<(), Problem>;

/// Runs a [Runner] and returns an [ExitCode].
///
/// If the runner is [Ok] it will return [ExitCode::SUCCESS].
///
/// Otherwise it will use an [ExitCode] if attached to the [Problem], defaulting to
/// [ExitCode::FAILURE]. Note that it possible to attach [ExitCode::SUCCESS].
///
/// The problem's [Display] representation will be printed in red to stderr.
pub fn run(run: Runner) -> ExitCode {
    match run() {
        Ok(_) => ExitCode::SUCCESS,

        Err(problem) => {
            let message = problem.to_string();
            if !message.is_empty() {
                eprintln!("{}", format!("{}", message.trim_end_matches('\n')).red());
            }

            problem.into()
        }
    }
}
