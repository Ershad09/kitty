use super::*;
use std::io::Cursor;

fn test_args() -> Args {
    Args {
        file: vec![],

        number: false,
        number_nonblank: false,
        squeeze_blank: false,

        show_ends: false,
        show_tabs: false,
        show_nonprinting: false,

        show_all: false,
        show_ends_nonprinting: false,
        show_tabs_nonprinting: false,
    }
}

#[test]
fn test_print_reader() {
    let input = b"hello\nworld\n";

    let reader = Cursor::new(input);

    let mut output = Vec::new();

    let args = test_args();

    print_reader(reader, &mut output, &args).unwrap();

    assert_eq!(output, b"hello\nworld\n");
}

// -n
#[test]
fn test_number_lines() {
    let input = b"hello\nworld\n";

    let reader = Cursor::new(input);
    let mut output = Vec::new();

    let mut args = test_args();
    args.number = true;

    print_reader(reader, &mut output, &args).unwrap();

    assert_eq!(output, b"     1\thello\n     2\tworld\n");
}

// -b
#[test]
fn test_number_nonblank_lines() {
    let input = b"hello\n\nworld\n";

    let reader = Cursor::new(input);

    let mut output = Vec::new();

    let mut args = test_args();
    args.number_nonblank = true;

    print_reader(reader, &mut output, &args).unwrap();

    assert_eq!(output, b"     1\thello\n\n     2\tworld\n");
}

// -b overrides -n
#[test]
fn test_number_nonblank_overrides_number() {
    let input = b"hello\n\nworld\n";

    let reader = Cursor::new(input);
    let mut output = Vec::new();

    let mut args = test_args();

    // -n -b
    args.number = true;
    args.number_nonblank = true;

    args.normalize();

    print_reader(reader, &mut output, &args).unwrap();

    assert_eq!(output, b"     1\thello\n\n     2\tworld\n");
}

//  -s
#[test]
fn test_squeeze_blank() {
    let input = b"hello\n\n\n\nworld\n";

    let reader = Cursor::new(input);

    let mut output = Vec::new();

    let mut args = test_args();
    args.squeeze_blank = true;

    print_reader(reader, &mut output, &args).unwrap();

    assert_eq!(output, b"hello\n\nworld\n");
}

//  -E
#[test]
fn test_show_ends() {
    let input = b"hello\nworld\n";

    let reader = Cursor::new(input);
    let mut output = Vec::new();

    let mut args = test_args();
    args.show_ends = true;

    print_reader(reader, &mut output, &args).unwrap();

    assert_eq!(output, b"hello$\nworld$\n");
}

//  -T
#[test]
fn test_show_tabs() {
    let input = b"hello\tworld\n";

    let reader = Cursor::new(input);
    let mut output = Vec::new();

    let mut args = test_args();
    args.show_tabs = true;

    print_reader(reader, &mut output, &args).unwrap();

    assert_eq!(output, b"hello^Iworld\n");
}

// -V
#[test]
fn test_show_nonprinting() {
    let input = [1u8, b'\n'];

    let reader = Cursor::new(input);
    let mut output = Vec::new();

    let mut args = test_args();
    args.show_nonprinting = true;

    print_reader(reader, &mut output, &args).unwrap();

    assert_eq!(output, b"^A\n");
}

//  -A
#[test]
fn test_show_all() {
    let input = [1u8, b'\t', b'a', b'\n'];

    let reader = Cursor::new(input);
    let mut output = Vec::new();

    let mut args = test_args();
    args.show_all = true;

    args.normalize();

    print_reader(reader, &mut output, &args).unwrap();

    assert_eq!(output, b"^A^Ia$\n");
}

// -e
#[test]
fn test_show_ends_nonprinting() {
    let input = [1u8, b'a', b'\n'];

    let reader = Cursor::new(input);
    let mut output = Vec::new();

    let mut args = test_args();
    args.show_ends_nonprinting = true;

    args.normalize();

    print_reader(reader, &mut output, &args).unwrap();

    assert_eq!(output, b"^Aa$\n");
}

//  -t
#[test]
fn test_show_tabs_nonprinting() {
    let input = [1u8, b'\t', b'a', b'\n'];

    let reader = Cursor::new(input);
    let mut output = Vec::new();

    let mut args = test_args();
    args.show_tabs_nonprinting = true;

    args.normalize();

    print_reader(reader, &mut output, &args).unwrap();

    assert_eq!(output, b"^A^Ia\n");
}

//  -n -s
#[test]
fn test_number_with_squeeze_blank() {
    let input = b"one\n\n\nthree\n";

    let reader = Cursor::new(input);
    let mut output = Vec::new();

    let mut args = test_args();

    //  -n -s
    args.number = true;
    args.squeeze_blank = true;

    print_reader(reader, &mut output, &args).unwrap();

    assert_eq!(output, b"     1\tone\n     2\t\n     3\tthree\n");
}

//  -b -s
#[test]
fn test_number_nonblank_with_squeeze() {
    let input = b"one\n\n\nthree\n";

    let reader = Cursor::new(input);
    let mut output = Vec::new();

    let mut args = test_args();

    // -b -s
    args.number_nonblank = true;
    args.squeeze_blank = true;

    args.normalize();

    print_reader(reader, &mut output, &args).unwrap();

    assert_eq!(output, b"     1\tone\n\n     2\tthree\n");
}

// -n -E
#[test]
fn test_number_and_show_ends() {
    let input = b"hello\nworld\n";

    let reader = Cursor::new(input);
    let mut output = Vec::new();

    let mut args = test_args();

    // -n -E
    args.number = true;
    args.show_ends = true;

    args.normalize();

    print_reader(reader, &mut output, &args).unwrap();

    assert_eq!(output, b"     1\thello$\n     2\tworld$\n");
}

//  -b -E
#[test]
fn test_number_nonblank_and_show_ends() {
    let input = b"hello\n\nworld\n";

    let reader = Cursor::new(input);
    let mut output = Vec::new();

    let mut args = test_args();

    // -b -E
    args.number_nonblank = true;
    args.show_ends = true;

    args.normalize();

    print_reader(reader, &mut output, &args).unwrap();

    assert_eq!(output, b"     1\thello$\n$\n     2\tworld$\n");
}

//  format_nonprinting()
#[test]
fn test_format_control_character() {
    let mut output = Vec::new();

    format_nonprinting(1, &mut output).unwrap();

    assert_eq!(output, b"^A");
}

#[test]
fn test_format_zero() {
    let mut output = Vec::new();

    format_nonprinting(0, &mut output).unwrap();

    assert_eq!(output, b"^@");
}

#[test]
fn test_format_delete() {
    let mut output = Vec::new();

    format_nonprinting(127, &mut output).unwrap();

    assert_eq!(output, b"^?");
}

#[test]
fn test_format_high_byte() {
    let mut output = Vec::new();

    format_nonprinting(129, &mut output).unwrap();

    assert_eq!(output, b"M-^A");
}
