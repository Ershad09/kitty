use crate::cli::Args;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Read, Write},
    path::Path,
};

pub fn print_file(path: &Path, args: &Args) -> io::Result<()> {
    let reader: Box<dyn Read> = if path == Path::new("-") {
        Box::new(io::stdin().lock())
    } else {
        Box::new(File::open(path)?)
    };

    let mut buffered_reader = BufReader::new(reader);

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    if !args.number && !args.number_nonblank {
        io::copy(&mut buffered_reader, &mut handle)?;
        return Ok(());
    }

    let mut line_number: u64 = 1;
    let mut line_buf = String::new();

    loop {
        line_buf.clear();
        let bytes_read = buffered_reader.read_line(&mut line_buf)?;
        if bytes_read == 0 {
            break;
        }

        let is_empty = line_buf == "\n" || line_buf == "\r\n";

        if args.number_nonblank {
            if !is_empty {
                write!(handle, "{:>6}\t{}", line_number, line_buf)?;
                line_number += 1;
            } else {
                write!(handle, "{}", line_buf)?;
            }
        } else if args.number {
            write!(handle, "{:>6}\t{}", line_number, line_buf)?;
            line_number += 1;
        }
    }

    Ok(())
}
