
use std::{
    fs::File,
    io::{self, BufReader},
    path::{Path, PathBuf},
};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "kitty",
    version,
    about = "Print UTF-8 file contents to standard output."
)]
pub struct Args {
    pub file: Vec<PathBuf>,
}

pub fn print_file(path: &Path) -> io::Result<()> {
    let file = File::open(path)?;

    let mut reader = BufReader::new(file);

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    io::copy(&mut reader, &mut handle)?;

    Ok(())
}
