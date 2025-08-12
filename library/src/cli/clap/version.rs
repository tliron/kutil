use clap::*;

build_info::build_info!(fn build_info);

//
// Version
//

/// Clap command to print version.
#[derive(Args, Clone, Debug)]
pub struct Version {
    /// show this help
    #[arg(long, short = 'h', action = ArgAction::Help)]
    pub help: Option<bool>,

    /// verbose
    #[arg(long, short = 'v')]
    verbose: bool,
}

impl Version {
    /// Run command.
    pub fn run<ParserT>(&self)
    where
        ParserT: Parser,
    {
        let command = ParserT::command();
        if let Some(version) = command.get_version() {
            println!("{}: {}", command.get_name(), version);
        }

        if self.verbose {
            let build_info = build_info();

            if let Some(version_control) = &build_info.version_control
                && let Some(git) = version_control.git()
            {
                println!();
                println!("git-commit-id: {}", git.commit_id);
                println!("git-commit-timestamp: {}", git.commit_timestamp);
                if let Some(branch) = &git.branch {
                    println!("git-commit-branch: {}", branch);
                }
                if !git.tags.is_empty() {
                    println!("git-commit-tags: {}", git.tags.join(","));
                }
                println!("git-dirty: {}", git.dirty);
            }

            println!();
            println!("binary-cpu: {}", build_info.target.cpu.arch);
            println!("binary-cpu-bits: {}", build_info.target.cpu.pointer_width);
            println!("binary-cpu-endianness: {}", build_info.target.cpu.endianness.to_string().to_lowercase());
            if !build_info.target.cpu.features.is_empty() {
                println!("binary-cpu-features: {}", build_info.target.cpu.features.join(","));
            }
            println!("binary-os: {}", build_info.target.os);
            println!("binary-architecture: {}", build_info.target.triple);

            println!();
            println!("compilation-timestamp: {}", build_info.timestamp);
            println!("compilation-profile: {}", build_info.profile);
            println!("compilation-optimization-level: {}", build_info.optimization_level);

            println!();
            println!("compiler: rustc");
            println!("compiler-version: {}", build_info.compiler.version);
            println!("compiler-channel: {}", build_info.compiler.channel.to_string().to_lowercase());
            if let Some(commit_id) = &build_info.compiler.commit_id {
                println!("compiler-git-commit-id: {}", commit_id);
            }
            if let Some(commit_date) = &build_info.compiler.commit_date {
                println!("compiler-git-commit-date: {}", commit_date);
            }
        }
    }
}
