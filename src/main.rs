#![deny(unsafe_code)]
use std::process;

use clap::Parser;

use toor::{cli::Opts, config::Config, exit_codes::ExitCode, output, project, Error};

fn main() {
    let result = run();
    match result {
        Ok(exit_code) => {
            process::exit(exit_code.into());
        }
        Err(err) => {
            output::stderr(&format!("Error: {err:?}"));
            process::exit(ExitCode::GeneralError.into());
        }
    }
}

fn run() -> miette::Result<ExitCode> {
    let opts = Opts::parse();
    let _config = Config {};

    let path = match opts.path {
        None => std::env::current_dir().unwrap(),
        Some(p) => p,
    };
    let root = project::find_project_root(&path);
    match root {
        Some(path) => {
            output::stdout(&path.display().to_string());
            Ok(ExitCode::Success)
        }
        None => Err(Error::RootNotFound.into()),
    }
}
