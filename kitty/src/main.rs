
use clap::Parser;
use kitty::{Args, print_file};
use std::{eprintln, process};

fn main() {
    let args = Args::parse();

    for file in args.file {
        if let Err(error) = print_file(&file) {
            eprintln!("Kitty: {}: {} ", file.display(), error);
            process::exit(1);
        }
    }
}
