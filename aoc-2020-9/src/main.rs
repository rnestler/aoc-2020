use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;

fn check_if_valid(preamble: &VecDeque<i64>, number: i64) -> bool {
    for (i, a) in preamble.iter().enumerate() {
        for b in preamble.iter().skip(i + 1) {
            if *a + *b == number {
                return true;
            }
        }
    }
    return false;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let lines = contents.lines();

    let mut preamble: VecDeque<i64> = lines.clone().take(25).map(|v| v.parse().unwrap()).collect();

    for line in lines.skip(25) {
        let number: i64 = line.parse().unwrap();
        if !check_if_valid(&preamble, number) {
            println!("Part 1: {}", number);
            break;
        }
        preamble.pop_front();
        preamble.push_back(number);
    }

    Ok(())
}
