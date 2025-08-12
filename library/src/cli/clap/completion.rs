use {clap::*, clap_complete_command::*, std::io};

//
// Completion
//

/// Clap command to generate shell auto-completion scripts.
///
/// See documentation for [bash](https://github.com/scop/bash-completion/blob/main/README.md).
///
/// To test with bash:
///
/// ```
/// mkdir --parents ~/.local/share/bash-completion/completions/
/// mycommand completion bash > ~/.local/share/bash-completion/completions/mycommand
/// reset
/// mycommand [press tab!]
/// ```
#[derive(Args, Clone, Debug)]
pub struct Completion {
    /// shell
    #[arg(value_enum)]
    shell: Shell,

    /// show this help
    #[arg(long, short = 'h', action = ArgAction::Help)]
    pub help: Option<bool>,
}

impl Completion {
    /// Run command.
    pub fn run<ParserT>(&self)
    where
        ParserT: Parser,
    {
        self.shell.generate(&mut ParserT::command(), &mut io::stdout());
    }
}
