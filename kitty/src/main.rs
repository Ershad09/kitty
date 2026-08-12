use clap::Parser;
use kitty::{cli::Args, io::print::print_file};
use std::{path::Path, process};

fn main() {
    let args = Args::parse();
    let mut exit_code = 0;

    for file in &args.file {
        if let Err(error) = print_file(file, &args) {
            if error.kind() == std::io::ErrorKind::BrokenPipe {
                process::exit(0);
            }

            let display_name = if file == Path::new("-") {
                "<stdin>"
            } else {
                file.to_str().unwrap_or("")
            };

            eprintln!("kitty: {}: {}", display_name, error);
            exit_code = 1;
        }
    }
    if exit_code != 0 {
        process::exit(exit_code);
    }
}
