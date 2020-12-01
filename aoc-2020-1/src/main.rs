use std::fs::File;
use std::io::prelude::*;

fn find_2_numbers_that_sum_to_2020(numbers: &Vec<i32>) {
    for (i, a) in numbers.iter().enumerate() {
        for b in &numbers[i+1..] {
            if a + b == 2020 {
                println!("{} * {} = {}", a, b, a * b);
            }
        }
    }
}

fn find_3_numbers_that_sum_to_2020(numbers: &Vec<i32>) {
    for (i, a) in numbers.iter().enumerate() {
        for (j, b) in numbers[i+1..].iter().enumerate() {
            if a + b >= 2020 {
                continue;
            }
            for c in &numbers[j+1..] {
                if a + b + c == 2020 {
                    println!("{} * {} * {} = {}", a, b, c, a * b * c);
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let numbers: Vec<i32> = contents
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();

    // Test
    //let numbers = vec![1721, 979, 366, 299, 675, 1456];

    find_2_numbers_that_sum_to_2020(&numbers);

    find_3_numbers_that_sum_to_2020(&numbers);

    Ok(())
}
