use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(())
}
