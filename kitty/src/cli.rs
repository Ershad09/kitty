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

    // -n
    #[arg(short = 'n', long = "number", help = "Number all output lines")]
    pub number: bool,

    //-b
    #[arg(
        short = 'b',
        long = "number-nonblank",
        help = "Number nonempty output lines"
    )]
    pub number_nonblank: bool,

    //-s
    #[arg(
        short = 's',
        long = "squeeze-blank",
        help = "Suppress repeated empty output lines"
    )]
    pub squeeze_blank: bool,

    // -E
    #[arg(
        short = 'E',
        long = "show-ends",
        help = "Display $ at end of each line"
    )]
    pub show_ends: bool,

    // -T
    #[arg(short = 'T', long = "show-tabs", help = "Display TAB characters as ^I")]
    pub show_tabs: bool,
}
