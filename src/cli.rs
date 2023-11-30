use clap::Parser;

#[derive(Parser)]
#[command(
    name = "toor",
    version,
    about = "toor 🫚. \nFind project root.",
    after_long_help = "Bugs can be reported on GitHub: https://github.com/azzamsa/toor/issues"
)]
pub struct Opts {}
