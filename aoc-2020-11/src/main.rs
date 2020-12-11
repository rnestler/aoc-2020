use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Field {
    Floor,
    Empty,
    Occupied,
}

impl From<u8> for Field {
    fn from(value: u8) -> Self {
        match value {
            b'L' => Field::Empty,
            b'.' => Field::Floor,
            b'#' => Field::Occupied,
            _ => panic!("Invalid value"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Map {
    map: Vec<Vec<Field>>,
}

impl From<&str> for Map {
    fn from(value: &str) -> Map {
        let map = value
            .lines()
            .map(|line| line.bytes().map(Field::from).collect())
            .collect();
        Map { map }
    }
}

impl Map {
    fn get_occupied_seats_at(&self, x: usize, y: usize) -> u32 {
        let mut occupied_seats = 0;
        if x > 0 {
            let column = &self.map[x - 1];
            if let Some(Field::Occupied) = column.get(y + 1) {
                occupied_seats += 1;
            }
            if let Some(Field::Occupied) = column.get(y) {
                occupied_seats += 1;
            }
            if y > 0 {
                if let Some(Field::Occupied) = column.get(y - 1) {
                    occupied_seats += 1;
                }
            }
        }
        let column = &self.map[x];
        if let Some(Field::Occupied) = column.get(y + 1) {
            occupied_seats += 1;
        }
        if y > 0 {
            if let Some(Field::Occupied) = column.get(y - 1) {
                occupied_seats += 1;
            }
        }

        if x < self.map.len() - 1 {
            let column = &self.map[x + 1];
            if let Some(Field::Occupied) = column.get(y + 1) {
                occupied_seats += 1;
            }
            if let Some(Field::Occupied) = column.get(y) {
                occupied_seats += 1;
            }
            if y > 0 {
                if let Some(Field::Occupied) = column.get(y - 1) {
                    occupied_seats += 1;
                }
            }
        }
        occupied_seats
    }

    fn check_position(&self, x: usize, y: usize) -> Field {
        let occupied_seats = self.get_occupied_seats_at(x, y);
        match self.map[x][y] {
            Field::Empty => {
                if occupied_seats == 0 {
                    Field::Occupied
                } else {
                    Field::Empty
                }
            }
            Field::Occupied => {
                if occupied_seats >= 4 {
                    Field::Empty
                } else {
                    Field::Occupied
                }
            }
            Field::Floor => Field::Floor,
        }
    }

    fn step(&mut self) -> bool {
        let mut new_map = self.map.clone();

        for x in 0..self.map.len() {
            for y in 0..self.map[0].len() {
                new_map[x][y] = self.check_position(x, y);
            }
        }
        if self.map == new_map {
            true
        } else {
            self.map = new_map;
            false
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut map = Map::from(contents.as_str());

    loop {
        let done = map.step();
        if done {
            break;
        }
    }
    let part_1: usize = map
        .map
        .iter()
        .map(|v| v.iter().filter(|&&x| x == Field::Occupied).count())
        .sum();
    println!("{}", part_1);

    Ok(())
}
