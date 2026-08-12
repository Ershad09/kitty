use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "kitty",
    version,
    about = "Print UTF-8 file contents to standard output."
)]
pub struct Args {
    #[arg(default_value = "-")]
    pub file: Vec<PathBuf>,

    #[arg(short = 'n', long = "number")]
    pub number: bool,

    #[arg(short = 'b', long = "number-nonblank")]
    pub number_nonblank: bool,
}
