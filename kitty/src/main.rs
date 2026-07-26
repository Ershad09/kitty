use clap::Parser;
use kitty::{Args, print_file};
use std::{eprintln, path::PathBuf, process};

fn main() {
    let mut args = Args::parse();

    if args.file.is_empty() {
        args.file.push(PathBuf::from("-"));
    }

    for file in &args.file {
        if let Err(error) = print_file(file) {
            let display_name = if file == &PathBuf::from("-") {
                "<stdin>".to_string()
            } else {
                file.display().to_string()
            };

            eprintln!("Kitty: {}: {}", display_name, error);
            process::exit(1);
        }
    }
}
