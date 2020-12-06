use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::iter::FromIterator;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let groups = contents.split("\n\n");

    let part1: usize = groups
        .clone()
        .map(|group| {
            let mut group: Vec<u8> = group.as_bytes().into();
            group.sort();
            group.iter().filter(|v| **v != b'\n').unique().count()
        })
        .sum();
    println!("Part 1: {}", part1);

    let part2: usize = groups
        .map(|group| {
            let people: Vec<HashSet<u8>> = group
                .lines()
                .map(|line| HashSet::from_iter(line.bytes()))
                .collect();

            let first = people[0].clone();
            people
                .iter()
                .fold(first, |acc, next| {
                    acc.intersection(&next).cloned().collect()
                })
                .iter()
                .count()
        })
        .sum();

    println!("Part 2: {}", part2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
}
