use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(
    name = "toor",
    version,
    about = "toor 🫚. \nFind project root.",
    after_long_help = "Bugs can be reported on GitHub: https://github.com/azzamsa/toor/issues"
)]
pub struct Opts {
    /// Optional custom path to start searching for the project root. If not provided,
    /// the search will start from the current working directory.
    #[arg(help = "Path to search", long_help)]
    pub path: Option<PathBuf>,
}
