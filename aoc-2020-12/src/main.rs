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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Ship {
    x: i32,
    y: i32,
    direction: Direction,
    waypoint: Point,
}

impl Ship {
    pub fn new() -> Ship {
        Ship {
            x: 0,
            y: 0,
            direction: Direction::East,
            waypoint: Point { x: 10, y: 1 },
        }
    }

    pub fn apply_action_part_1(&mut self, action: Action) {
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

    pub fn apply_action_part_2(&mut self, action: Action) {
        match action {
            Action::North(value) => self.waypoint.y += value,
            Action::South(value) => self.waypoint.y -= value,
            Action::East(value) => self.waypoint.x += value,
            Action::West(value) => self.waypoint.x -= value,
            Action::Left(0) | Action::Right(0) => {}
            Action::Left(90) | Action::Right(270) => {
                let new_point = Point {
                    x: -self.waypoint.y,
                    y: self.waypoint.x,
                };
                self.waypoint = new_point;
            }
            Action::Left(180) | Action::Right(180) => {
                let new_point = Point {
                    x: -self.waypoint.x,
                    y: -self.waypoint.y,
                };
                self.waypoint = new_point;
            }
            Action::Left(270) | Action::Right(90) => {
                let new_point = Point {
                    x: self.waypoint.y,
                    y: -self.waypoint.x,
                };
                self.waypoint = new_point;
            }
            Action::Left(value) | Action::Right(value) => {
                panic!("Invalid value: {}", value);
            }
            Action::Forward(value) => {
                self.y += self.waypoint.y * value;
                self.x += self.waypoint.x * value;
            }
        }
    }

    pub fn part_1(&mut self, input: &str) -> i32 {
        for line in input.lines() {
            let action = Action::from(line);
            self.apply_action_part_1(action);
        }
        self.x.abs() + self.y.abs()
    }

    pub fn part_2(&mut self, input: &str) -> i32 {
        for line in input.lines() {
            let action = Action::from(line);
            self.apply_action_part_2(action);
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

    let mut ship = Ship::new();
    let part_2 = ship.part_2(&contents);
    println!("Part 2: {}", part_2);
    // 167123 too high

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "F10\nN3\nF7\nR90\nF11";

    #[test]
    fn test_part_1() {
        let mut ship = Ship::new();
        let part_1 = ship.part_1(TEST_INPUT);
        assert_eq!(part_1, 25);
    }

    #[test]
    fn test_part_2() {
        let mut ship = Ship::new();
        let part_2 = ship.part_2(TEST_INPUT);
        assert_eq!(part_2, 286);
    }
}
