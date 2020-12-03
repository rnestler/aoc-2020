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

    fn part_1(&self) -> usize {
        let mut x = 0;
        let mut y = 0;
        let mut tree_count = 0;

        loop {
            if self.check_position(x, y) {
                tree_count += 1;
            }
            x += 3;
            y += 1;
            if y >= self.trees.len() {
                break;
            }
        }
        tree_count
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let map = Map::from(contents.as_str());
    println!("part1: {}", map.part_1());
    Ok(())
}
