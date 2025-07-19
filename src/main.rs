use std::path::Component;
use std::process::ExitCode;

use clap::Parser as ClapParser;
use regex::Regex;
use walkdir::WalkDir;

#[derive(ClapParser)]
struct Command {
    dir_path: String,
    regex: String,
    /// Allow search dot directories
    #[arg(long = "dot_dir", short = 'D')]
    allow_search_dot_dirs: bool,
}

macro_rules! unwrap {
    ($expr: expr, $msg: expr) => {
        match $expr {
            Ok(ok) => ok,
            Err(err) => {
                eprintln!("ERROR: {}", $msg);
                eprintln!("REASON: {err}");
                return ExitCode::FAILURE;
            }
        }
    };
}

fn main() -> ExitCode {
    let command = Command::parse();
    let regex = unwrap!(Regex::new(&command.regex), "failed to initalize regex");

    for entry in WalkDir::new(command.dir_path) {
        let entry = unwrap!(entry, "failed to obtain entry");

        if entry.path().components().any(|comp| match comp {
            Component::Normal(os_str) => os_str
                .to_str()
                .map(|s| s.as_bytes()[0] == b'.')
                .unwrap_or(false),
            _ => false,
        }) {
            continue;
        }

        let Some(basename) = entry.path().to_str() else {
            return ExitCode::FAILURE;
        };

        if regex.is_match(basename) {
            println!("{basename}");
        }
    }

    ExitCode::SUCCESS
}
