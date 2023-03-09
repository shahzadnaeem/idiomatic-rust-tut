use std::io::ErrorKind;

use crate::file_parse::{self, NumberFromFileError};

#[test]
pub fn nothing_if_empty() {
    let data = "".to_string();

    let err = file_parse::parse_number(data);

    assert!(matches!(err, Err(NumberFromFileError::EmptyFile)));
}

#[test]
pub fn nothing_if_blank_lines() {
    let data = "\n\n\n".to_string();

    let err = file_parse::parse_number(data);

    assert!(matches!(err, Err(NumberFromFileError::EmptyFile)));
}

#[test]
pub fn nothing_if_comments() {
    let data = "// Just a comment\n# Other kind of comment".to_string();

    let err = file_parse::parse_number(data);

    assert!(matches!(err, Err(NumberFromFileError::EmptyFile)));
}

#[test]
pub fn first_value_returned() {
    let data = "1\n2".to_string();

    let res = file_parse::parse_number(data).unwrap();

    assert_eq!(1, res);
}

#[test]
pub fn num_with_underscores_ok() {
    let data = "1_00\n#Too far 123".to_string();

    let res = file_parse::parse_number(data).unwrap();

    assert_eq!(100, res);
}

fn is_io_error_of_kind(error: NumberFromFileError, kind: ErrorKind) {
    match error {
        NumberFromFileError::IoError(e) => {
            assert_eq!(e.kind(), kind);
        }
        _ => assert!(false),
    };
}

#[test]
pub fn missing_file_detected() {
    let err = file_parse::read_number_from_file("not_a_file").unwrap_err();

    is_io_error_of_kind(err, ErrorKind::NotFound);
}

#[test]
pub fn gets_value_from_file() {
    let num = file_parse::read_number_from_file("src/data/data.txt").unwrap();

    assert_eq!(1007, num);
}
