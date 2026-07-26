use clap::Parser;
use std::{
    fs::File,
    io::{self, BufReader, Read},
    path::{Path, PathBuf},
};

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
    
    let reader: Box<dyn Read> = if path == Path::new("-") {
        Box::new(io::stdin().lock())
    } else {
        Box::new(File::open(path)?)
    };

    let mut buf_reader = BufReader::new(reader);
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    io::copy(&mut buf_reader, &mut handle)?;

    Ok(())
}