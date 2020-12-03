use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct Map {
    trees: Vec<Vec<bool>>,
}

impl From<&str> for Map {
    fn from(value: &str) -> Map {
        let trees = value
            .lines()
            .map(|line| {
                line.bytes()
                    .map(|byte| if byte == b'#' { true } else { false })
                    .collect()
            })
            .collect();
        Map { trees }
    }
}

impl Map {
    pub fn check_position(&self, x: usize, y: usize) -> bool {
        // we assume the map is rectangular
        let x = x % self.trees[0].len();
        self.trees[y][x]
    }

    fn check_slope(&self, dx: usize, dy: usize) -> usize {
        let mut x = 0;
        let mut y = 0;
        let mut tree_count = 0;

        loop {
            if self.check_position(x, y) {
                tree_count += 1;
            }
            x += dx;
            y += dy;
            if y >= self.trees.len() {
                break;
            }
        }
        tree_count
    }

    fn part_1(&self) -> usize {
        self.check_slope(3, 1)
    }

    fn part_2(&self) -> usize {
        let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
        slopes.iter().fold(1, |product, slope| {
            product * self.check_slope(slope.0, slope.1)
        })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let map = Map::from(contents.as_str());
    println!("part1: {}", map.part_1());
    println!("part1: {}", map.part_2());
    Ok(())
}
