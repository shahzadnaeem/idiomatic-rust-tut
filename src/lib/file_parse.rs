use std::{fs::File, io::Read, num::ParseIntError};

// enum to handle all possible errors - both sourse errors and one of our own
#[derive(Debug)]
pub enum NumberFromFileError {
    ParseError(ParseIntError),
    IoError(std::io::Error),
    EmptyFile,
}

fn priv_fn<'a>(a: u32, b: u32) -> Result<u32, &'a str> {
    if u32::MAX - b > a {
        Ok(a + b)
    } else {
        Err("Nah!")
    }
}

pub fn _dummy<'a>() -> Result<u32, &'a str> {
    priv_fn(1, 2)
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
            NumberFromFileError::EmptyFile => {
                write!(f, "No number in file!")
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

pub fn parse_number(data: String) -> Result<u64, NumberFromFileError> {
    for line in data.lines() {
        let trimmed = line.trim();

        if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with("//") {
            continue;
        }

        // Let's remove '_' so we can parse things like 1_000
        let trimmed = trimmed.chars().filter(|c| *c != '_').collect::<String>();

        let parsed = trimmed.parse()?;

        return Ok(parsed);
    }

    Err(NumberFromFileError::EmptyFile)
}

pub fn read_number_from_file(filename: &str) -> Result<u64, NumberFromFileError> {
    let data = read_file(filename)?;

    let parsed = parse_number(data)?;

    Ok(parsed)
}

// Tests ----------------------------------------------------------------------

#[cfg(test)]
#[path = "file_parse/file_parse_tests.rs"]
mod file_parse_tests;
