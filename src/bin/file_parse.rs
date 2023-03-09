use std::error::Error;

use idioms::file_parse::{read_file, read_number_from_file};

pub fn main() -> Result<(), Box<dyn Error>> {
    const DATA_PATH: &str = "src/data/data.txt";

    let data = read_file(DATA_PATH)?;

    println!("\nFile: {} contents:{}\n", DATA_PATH, data);

    let num = read_number_from_file(DATA_PATH);

    match num {
        Ok(n) => println!("Number read: {}", n),
        Err(e) => println!("Error: {}", e),
    }

    Ok(())
}
