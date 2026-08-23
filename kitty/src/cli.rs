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

    // -n -> shows number all output lines
    #[arg(short = 'n', long = "number", help = "Number all output lines")]
    pub number: bool,

    //-b -> shows bumber nonempty output lines
    #[arg(
        short = 'b',
        long = "number-nonblank",
        help = "Number nonempty output lines"
    )]
    pub number_nonblank: bool,

    //-s -> suppress repeated empty output lines
    #[arg(
        short = 's',
        long = "squeeze-blank",
        help = "Suppress repeated empty output lines"
    )]
    pub squeeze_blank: bool,

    // -E -> display $ at end of each line
    #[arg(
        short = 'E',
        long = "show-ends",
        help = "Display $ at end of each line"
    )]
    pub show_ends: bool,

    // -T -> display TAB characters as ^I
    #[arg(short = 'T', long = "show-tabs", help = "Display TAB characters as ^I")]
    pub show_tabs: bool,

    // -v -> use ^ and M- notation, except for LFD and TAB
    #[arg(short = 'v', long = "show-nonprinting")]
    pub show_nonprinting: bool,

    //////////// ---------------------

    // -A ->  equivalent to -vET
    #[arg(short = 'A', long = "show-all")]
    pub show_all: bool,

    // -e -> equivalent to -vE
    #[arg(short = 'e')]
    pub show_ends_nonprinting: bool,

    // -t -> equivalent to -vT
    #[arg(short = 't')]
    pub show_tabs_nonprinting: bool,
}

impl Args {
    /// normalizes composite flags (-A, -e, -t) into their base flags.
    pub fn normalize(&mut self) {
        if self.show_all {
            self.show_nonprinting = true;
            self.show_ends = true;
            self.show_tabs = true;
        }
        if self.show_ends_nonprinting {
            self.show_nonprinting = true;
            self.show_ends = true;
        }
        if self.show_tabs_nonprinting {
            self.show_nonprinting = true;
            self.show_tabs = true;
        }
        if self.number_nonblank {
            self.number = false;
        }
    }
}
