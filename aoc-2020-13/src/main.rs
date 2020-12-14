use std::fs::File;
use std::io::prelude::*;

fn part_2(input: &str) -> usize {
    let mut buses: Vec<(usize, usize)> = input
        .split(',')
        .enumerate()
        .filter_map(|s| s.1.parse().ok().map(|parsed| (s.0, parsed)))
        .collect();

    buses.sort_by(|a, b| b.1.cmp(&a.1));

    'outer: for t in 1.. {
        let t = t * buses[0].1 - buses[0].0;
        for bus in buses[1..].iter() {
            if (t + bus.0) % bus.1 != 0 {
                continue 'outer;
            }
        }
        return t;
    }
    panic!("No solution found!");
}

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

    let input = contents.lines().skip(1).next().unwrap();

    let part_2 = part_2(input);
    println!("Part 2: {}", part_2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUTS: [&str; 6] = [
        "17,x,13,19",
        "67,7,59,61",
        "67,x,7,59,61",
        "67,7,x,59,61",
        "1789,37,47,1889",
        "7,13,x,x,59,x,31,19",
    ];
    const RESULTS: [usize; 6] = [3417, 754018, 779210, 1261476, 1202161486, 1068781];

    #[test]
    fn test_part_2() {
        for (input, result) in TEST_INPUTS.iter().zip(&RESULTS) {
            let test_result = part_2(input);
            assert_eq!(test_result, *result);
        }
    }
}
