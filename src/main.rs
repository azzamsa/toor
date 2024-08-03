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

    let current_dir = match std::env::current_dir() {
        Ok(current_dir) => current_dir,
        Err(e) => return Err(crate::Error::Internal(e.to_string()).into()),
    };

    let path = match &opts.path {
        Some(path) => path,
        None => &current_dir,
    };

    let config = construct_config(opts.clone())?;
    let root = project::find_project_root(path.to_path_buf(), config);
    match root {
        Some(path) => {
            output::stdout(&path.display().to_string());
            Ok(ExitCode::Success)
        }
        None => Err(Error::RootNotFound.into()),
    }
}

fn construct_config(opts: Opts) -> Result<Config, crate::Error> {
    Ok(Config {
        root_pattern: opts.root_pattern,
    })
}
