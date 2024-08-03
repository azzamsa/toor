use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(
    name = "toor",
    version,
    about = "toor 🫚. \nFind project root.",
    after_long_help = "Bugs can be reported on GitHub: https://github.com/azzamsa/toor/issues"
)]
pub struct Opts {
    /// Path to search
    ///
    /// Optional custom path to start searching for the project root. If not provided,
    /// the search will start from the current working directory.
    pub path: Option<PathBuf>,

    /// Replace default root pattern
    ///
    /// Optional pattern to replace default root pattern.
    /// Usage: `--root-pattern ".project,.git`
    #[arg(short, long = "roots", value_delimiter = ',')]
    pub root_pattern: Option<Vec<String>>,
}
