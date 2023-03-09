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
