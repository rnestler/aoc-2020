use std::fs::File;
use std::io::prelude::*;
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Action {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

impl From<&str> for Action {
    fn from(value: &str) -> Action {
        let (action, value) = value.split_at(1);

        let value = value.parse().unwrap();

        match action {
            "N" => Action::North(value),
            "E" => Action::East(value),
            "S" => Action::South(value),
            "W" => Action::West(value),
            "L" => Action::Left(value),
            "R" => Action::Right(value),
            "F" => Action::Forward(value),
            _ => panic!("invalid action"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn to_degrees(&self) -> i32 {
        match self {
            Direction::North => 0,
            Direction::East => 90,
            Direction::South => 180,
            Direction::West => 270,
        }
    }

    fn from_degrees(degrees: i32) -> Direction {
        let degrees = degrees % 360;
        match degrees {
            -90 => Direction::West,
            -180 => Direction::South,
            -270 => Direction::East,
            0 => Direction::North,
            90 => Direction::East,
            180 => Direction::South,
            270 => Direction::West,
            _ => panic!("No valid direction: {}", degrees),
        }
    }

    fn turn_left(&mut self, degrees: i32) {
        let degrees = self.to_degrees() - degrees;
        *self = Direction::from_degrees(degrees);
    }
    fn turn_right(&mut self, degrees: i32) {
        let degrees = self.to_degrees() + degrees;
        *self = Direction::from_degrees(degrees);
    }
}

struct Ship {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Ship {
    pub fn new() -> Ship {
        Ship {
            x: 0,
            y: 0,
            direction: Direction::East,
        }
    }

    pub fn apply_action(&mut self, action: Action) {
        match action {
            Action::North(value) => self.y += value,
            Action::South(value) => self.y -= value,
            Action::East(value) => self.x += value,
            Action::West(value) => self.x -= value,
            Action::Left(value) => self.direction.turn_left(value),
            Action::Right(value) => self.direction.turn_right(value),
            Action::Forward(value) => match self.direction {
                Direction::North => self.y += value,
                Direction::South => self.y -= value,
                Direction::East => self.x += value,
                Direction::West => self.x -= value,
            },
        }
    }

    pub fn part_1(&mut self, input: &str) -> i32 {
        for line in input.lines() {
            let action = Action::from(line);
            self.apply_action(action);
        }
        self.x.abs() + self.y.abs()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut ship = Ship::new();
    let part_1 = ship.part_1(&contents);
    println!("Part 1: {}", part_1);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
}
