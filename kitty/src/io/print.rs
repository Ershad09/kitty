use std::{
    fs::File,
    io::{self, BufReader, Read},
    path::Path,
};

pub fn print_file(path: &Path) -> io::Result<()> {
    let mut reader: Box<dyn Read> = if path == Path::new("-") {
        Box::new(io::stdin().lock())
    } else {
        Box::new(BufReader::new(File::open(path)?))
    };

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    io::copy(&mut reader, &mut handle)?;

    Ok(())
}