use std::fs::File;
use std::io::prelude::*;

fn read_voltages(input: &str) -> Vec<u32> {
    input.lines().map(|v| v.parse().unwrap()).collect()
}

fn find_shortest_path(voltages: &[u32]) -> u32 {
    let mut cnt = 0;
    let mut current_joltage = 0;

    //let mut iter = voltages.iter();
    //iter.clone().take_while(|&x| x - current_joltage <= 3);
    //println!("{:?}", iter.next());

    for i in 0..(voltages.len() - 1) {
        // skip if next works
        if voltages[i + 1] - current_joltage <= 3 {
            continue;
        }
        current_joltage = voltages[i];
        cnt += 1;
    }

    // add last adapter
    cnt + 1
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
    for adapter in adapters.iter() {
        let diff = adapter - current_joltage;
        if diff == 1 {
            one_jolt_diff += 1;
        } else if diff == 3 {
            three_jolt_diff += 1;
        }
        if diff > 3 {
            panic!();
        }
        current_joltage = *adapter;
    }
    // device jolts = max + 3
    three_jolt_diff += 1;

    println!("Part 1: {}", one_jolt_diff * three_jolt_diff);

    let adapters = [1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19];
    /*let adapters = [
        1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31, 32, 33, 34, 35, 38,
        39, 42, 45, 46, 47, 48, 49,
    ];*/

    let diffs: Vec<_> = adapters
        .iter()
        .zip(&adapters[1..])
        .map(|(a, b)| b - a)
        .collect();

    println!("{:?}", &adapters[0..10]);
    println!("{:?}", &diffs[0..10]);

    let shortest = find_shortest_path(&adapters);
    dbg!(shortest);
    let bits = adapters.len() as u32 - shortest as u32;
    dbg!(bits);

    let result = 2u128.pow(bits);
    println!("{:?}", result);

    Ok(())
}
