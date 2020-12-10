use std::fs::File;
use std::io::prelude::*;

fn read_voltages(input: &str) -> Vec<u32> {
    input.lines().map(|v| v.parse().unwrap()).collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut adapters = read_voltages(&contents);
    adapters.sort();

    let mut one_jolt_diff = 0;
    let mut three_jolt_diff = 0;
    // charging outlet 0 jolts
    let mut current_joltage = 0;
    for adapter in adapters {
        let diff = adapter - current_joltage;
        if diff == 1 {
            one_jolt_diff += 1;
        } else if diff == 3 {
            three_jolt_diff += 1;
        }
        if diff > 3 {
            panic!();
        }
        current_joltage = adapter;
    }
    // device jolts = max + 3
    three_jolt_diff += 1;

    println!("Part 1: {}", one_jolt_diff * three_jolt_diff);

    Ok(())
}
