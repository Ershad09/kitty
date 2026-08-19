use crate::cli::Args;
use std::{
    borrow::Cow,
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

    if !args.number
        && !args.number_nonblank
        && !args.squeeze_blank
        && !args.show_ends
        && !args.show_tabs
    {
        io::copy(&mut buffered_reader, &mut handle)?;
        return Ok(());
    }

    let mut line_number: u64 = 1;
    let mut line_buf = String::new();
    let mut prev_was_empty = false;

    loop {
        line_buf.clear();
        let bytes_read = buffered_reader.read_line(&mut line_buf)?;
        if bytes_read == 0 {
            break;
        }

        let is_empty = line_buf == "\n" || line_buf == "\r\n";

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

        let mut output_line: Cow<str> = Cow::Borrowed(&line_buf);

        if args.show_tabs && output_line.contains('\t') {
            output_line = Cow::Owned(output_line.replace('\t', "^I"));
        }

        if args.show_ends {
            let mut owned = output_line.into_owned();
            if owned.ends_with("\r\n") {
                owned.replace_range(owned.len() - 2.., "$\r\n");
            } else if owned.ends_with('\n') {
                owned.replace_range(owned.len() - 1.., "$\n");
            } else {
                owned.push('$');
            }
            output_line = Cow::Owned(owned);
        }

        if args.number_nonblank {
            if !is_empty {
                write!(handle, "{:>6}\t{}", line_number, output_line)?;
                line_number += 1;
            } else {
                write!(handle, "{}", output_line)?;
            }
        } else if args.number {
            write!(handle, "{:>6}\t{}", line_number, output_line)?;
            line_number += 1;
        } else {
            write!(handle, "{}", output_line)?;
        }
    }

    Ok(())
}
