#![deny(unsafe_code)]
use std::process;

use clap::Parser;

use toor::{cli::Opts, config::Config, exit_codes::ExitCode, output};

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
    let _opts = Opts::parse();
    let _config = Config {};

    Ok(ExitCode::Success)
}
