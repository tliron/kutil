use {
    clap::*,
    clap_mangen::*,
    std::{io, path::*},
};

//
// Manual
//

/// Clap command to generate manual pages in the troff format.
///
/// Manual pages are usually located in "/usr/share/man/" (for everybody) or "~/.local/share/man/"
/// (for the current user). User commands are under the "man1/" subdirectory.
///
/// To see all supported paths:
///
/// ```
/// man --where
/// ```
///
/// To test:
///
/// ```
/// mkdir --parents ~/.local/share/man/man1/
/// mycommand manual ~/.local/share/man/man1/
/// man mycommand
/// ```
#[derive(Args, Clone, Debug)]
pub struct Manual {
    /// output path
    #[arg(verbatim_doc_comment, default_value = "/usr/share/man/man1/")]
    pub path: PathBuf,

    /// show this help
    #[arg(long, short = 'h', action = ArgAction::Help)]
    pub help: Option<bool>,
}

impl Manual {
    /// Run command.
    pub fn run<ParserT>(&self) -> io::Result<()>
    where
        ParserT: Parser,
    {
        generate_to(ParserT::command(), &self.path)
    }
}
