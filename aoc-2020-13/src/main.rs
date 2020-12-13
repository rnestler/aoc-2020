use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut lines = contents.lines();
    let earliest_start: u32 = lines.next().unwrap().parse().unwrap();
    let buses: Vec<u32> = lines
        .next()
        .unwrap()
        .split(',')
        .filter_map(|s| s.parse().ok())
        .collect();

    let min = buses
        .iter()
        .map(|bus| (earliest_start / bus + 1) * bus)
        .enumerate()
        .min_by_key(|x| x.1)
        .unwrap();

    println!("Part 1: {}", buses[min.0] * (min.1 - earliest_start));

    Ok(())
}
