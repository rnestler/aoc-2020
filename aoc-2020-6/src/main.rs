use itertools::Itertools;
use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let groups = contents.split("\n\n");

    let part1: usize = groups
        .map(|group| {
            let mut group: Vec<u8> = group.as_bytes().into();
            group.sort();
            group
                .iter()
                .filter(|v| **v != b'\n')
                .map(|v| u32::from(*v))
                .unique()
                .count()
        })
        .sum();
    println!("Part 1: {}", part1);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
}
