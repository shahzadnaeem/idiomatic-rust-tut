use std::{error::Error, fs::File, io::Read, num::ParseIntError};

pub fn main() -> Result<(), Box<dyn Error>> {
    // foo("bar", None, None, None);
    // foo("bar", Some(42), None, None);
    // foo("bar", Some(42), Some(1337), Some(-1));

    // foo2("bar", None, None, None);
    // foo2("bar", 42, None, None);
    // foo2("bar", 42, 1337, -1);

    // let num = read_number_from_file("data.txt")?;

    // println!("Number read: {}", num);

    let mut counter = Counter::new(3);

    let nums = counter.iter(10).collect::<Vec<_>>();

    println!("nums: {:?}, counter: {:?}", nums, counter);

    Ok(())
}

pub fn foo(lorem: &str, ipsum: Option<i32>, dolor: Option<i32>, sit: Option<i32>) {
    println!("{}, {:?}, {:?}, {:?}", lorem, ipsum, dolor, sit);
}

pub fn foo2<I, D, S>(lorem: &str, ipsum: I, dolor: D, sit: S)
where
    I: Into<Option<i32>> + std::fmt::Debug,
    D: Into<Option<i32>> + std::fmt::Debug,
    S: Into<Option<i32>> + std::fmt::Debug,
{
    println!("{}, {:?}, {:?}, {:?}", lorem, ipsum, dolor, sit);
}

// Create an enum for the possible errors we could get

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

pub fn read_number_from_file(filename: &str) -> Result<u64, NumberFromFileError> {
    let mut file = File::open(filename)?;
    let mut buffer = String::new();

    file.read_to_string(&mut buffer)?;

    let parsed: u64 = buffer.trim().parse()?;

    Ok(parsed)
}

#[derive(Debug)]
pub struct Counter {
    c: u32,
}

impl Default for Counter {
    fn default() -> Self {
        Counter { c: 0 }
    }
}

impl Counter {
    fn new(c: u32) -> Self {
        Counter { c }
    }

    // FIXME: Is this correct?
    fn iter(&mut self, to: u32) -> CounterIter {
        CounterIter::new(self, to)
    }
}

#[derive(Debug)]
pub struct CounterIter<'a> {
    counter: &'a mut Counter,
    to: u32,
}

impl<'a> CounterIter<'a> {
    pub fn new(counter: &'a mut Counter, to: u32) -> Self {
        CounterIter { counter, to }
    }
}

impl<'it> Iterator for CounterIter<'it> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let mut ctr = &mut self.counter;
        if ctr.c < self.to {
            let c = ctr.c;

            ctr.c += 1;

            Some(c)
        } else {
            None
        }
    }
}
