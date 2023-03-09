use std::{fs::File, io::Read, num::ParseIntError};

// enum to handle both possible errors
#[derive(Debug)]
pub enum NumberFromFileError {
    ParseError(ParseIntError),
    IoError(std::io::Error),
}

impl std::fmt::Display for NumberFromFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NumberFromFileError::IoError(io) => {
                write!(f, "{io}")
            }
            NumberFromFileError::ParseError(parse) => {
                write!(f, "{parse}")
            }
        }
    }
}

impl From<ParseIntError> for NumberFromFileError {
    fn from(err: ParseIntError) -> Self {
        NumberFromFileError::ParseError(err)
    }
}

impl From<std::io::Error> for NumberFromFileError {
    fn from(err: std::io::Error) -> Self {
        NumberFromFileError::IoError(err)
    }
}

impl std::error::Error for NumberFromFileError {}

pub fn read_file(filename: &str) -> Result<String, NumberFromFileError> {
    let mut file = File::open(filename)?;
    let mut buffer = String::new();

    file.read_to_string(&mut buffer)?;

    Ok(buffer)
}

pub fn read_number_from_file(filename: &str) -> Result<u64, NumberFromFileError> {
    let buffer = read_file(filename)?;

    let parsed: u64 = buffer.trim().parse()?;

    Ok(parsed)
}
