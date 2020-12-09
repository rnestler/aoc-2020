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
    let numbers: Vec<i64> = lines.map(|v| v.parse().unwrap()).collect();

    let mut preamble = VecDeque::<i64>::new();
    preamble.extend(&numbers[0..25]);

    let mut part_1 = 0;
    for number in numbers.iter().skip(25) {
        if !check_if_valid(&preamble, *number) {
            println!("Part 1: {}", number);
            part_1 = *number;
            break;
        }
        preamble.pop_front();
        preamble.push_back(*number);
    }

    for i in 0..numbers.len() {
        let mut sum = 0;
        let mut j = i;
        while sum < part_1 {
            sum += numbers[j];
            if sum == part_1 {
                let min = numbers[i..j].iter().min().unwrap();
                let max = numbers[i..j].iter().max().unwrap();
                println!("Part 2: {}", min + max);
                return Ok(());
            }
            j += 1;
        }
    }

    Ok(())
}
