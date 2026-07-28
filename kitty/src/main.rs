use clap::Parser;
use kitty::{cli::Args, io::print::print_file};
use std::{path::Path, process};

fn main() {
    let args = Args::parse();

    for file in &args.file {
        if let Err(error) = print_file(file) {
            let display_name = if file == Path::new("-") {
                "<stdin>"
            } else {
                file.to_str().unwrap_or("")
            };

            eprintln!("kitty: {}: {}", display_name, error);
            process::exit(1);
        }
    }
}