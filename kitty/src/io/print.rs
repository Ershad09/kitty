use crate::cli::Args;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, BufWriter, Read, Write},
    path::Path,
};

pub fn print_file(path: &Path, args: &Args) -> io::Result<()> {
    let reader: Box<dyn Read> = if path == Path::new("-") {
        Box::new(io::stdin().lock())
    } else {
        Box::new(File::open(path)?)
    };

    let stdout = io::stdout();

    if !args.number
        && !args.number_nonblank
        && !args.squeeze_blank
        && !args.show_ends
        && !args.show_tabs
        && !args.show_nonprinting
    {
        let mut mut_reader = reader;
        let mut handle = stdout.lock();

        io::copy(&mut mut_reader, &mut handle)?;
        return Ok(());
    }

    let mut buffered_reader = BufReader::new(reader);
    let mut writer = BufWriter::new(stdout.lock());

    let mut line_number: u64 = 1;
    let mut line_buf = Vec::new();
    let mut prev_was_empty = false;

    loop {
        line_buf.clear();
        let bytes_read = buffered_reader.read_until(b'\n', &mut line_buf)?;
        if bytes_read == 0 {
            break;
        }

        let is_empty = line_buf == b"\n" || line_buf == b"\r\n";

        if args.squeeze_blank {
            if is_empty {
                if prev_was_empty {
                    continue;
                }
                prev_was_empty = true;
            } else {
                prev_was_empty = false;
            }
        }

        if args.number_nonblank {
            if !is_empty {
                write!(writer, "{:>6}\t", line_number)?;
                line_number += 1;
            }
        } else if args.number {
            write!(writer, "{:>6}\t", line_number)?;
            line_number += 1;
        }

        for &byte in &line_buf {
            match byte {
                b'\t' => {
                    if args.show_tabs {
                        writer.write_all(b"^I")?;
                    } else {
                        writer.write_all(&[b'\t'])?;
                    }
                }
                b'\n' => {
                    if args.show_ends {
                        writer.write_all(b"$\n")?;
                    } else {
                        writer.write_all(&[b'\n'])?;
                    }
                }
                _ => {
                    if args.show_nonprinting {
                        format_nonprinting(byte, &mut writer)?;
                    } else {
                        writer.write_all(&[byte])?;
                    }
                }
            }
        }
    }

    writer.flush()?;

    Ok(())
}

fn format_nonprinting<W: Write>(byte: u8, writer: &mut W) -> io::Result<()> {
    let mut b = byte;
    if b >= 128 {
        writer.write_all(b"M-")?;
        b -= 128;
    }

    if b < 32 && b != b'\n' && b != b'\t' {
        writer.write_all(&[b'^', b + 64])?;
    } else if b == 127 {
        writer.write_all(b"^?")?;
    } else {
        writer.write_all(&[b])?;
    }
    Ok(())
}
