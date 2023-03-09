use std::{fs::File, io::Read, num::ParseIntError};

// enum to handle both possible errors
#[derive(Debug)]
pub enum NumberFromFileError {
    ParseError(ParseIntError),
    IoError(std::io::Error),
    EmptyFile,
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

fn parse_number(data: String) -> Result<u64, NumberFromFileError> {
    for line in data.lines() {
        let trimmed = line.trim();

        if trimmed.len() == 0 || trimmed.starts_with("#") || trimmed.starts_with("//") {
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
