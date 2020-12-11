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
    fn get_seat_at(&self, x: isize, y: isize) -> Option<Field> {
        if x < 0 {
            None
        } else if y < 0 {
            None
        } else {
            Some(*self.map.get(x as usize)?.get(y as usize)?)
        }
    }

    fn is_occupied_seat_in_direction(
        &self,
        mut x: isize,
        mut y: isize,
        dx: isize,
        dy: isize,
    ) -> bool {
        x += dx;
        y += dy;
        while let Some(seat) = self.get_seat_at(x, y) {
            x += dx;
            y += dy;
            if seat == Field::Occupied {
                return true;
            } else if seat == Field::Empty {
                return false;
            }
        }
        false
    }

    fn get_occupied_seats_at_part2(&self, x: usize, y: usize) -> u32 {
        let x = x as isize;
        let y = y as isize;
        let mut occupied_seats = 0;
        if self.is_occupied_seat_in_direction(x, y, -1, -1) {
            occupied_seats += 1;
        }
        if self.is_occupied_seat_in_direction(x, y, -1, 0) {
            occupied_seats += 1;
        }
        if self.is_occupied_seat_in_direction(x, y, -1, 1) {
            occupied_seats += 1;
        }
        if self.is_occupied_seat_in_direction(x, y, 0, -1) {
            occupied_seats += 1;
        }
        if self.is_occupied_seat_in_direction(x, y, 0, 1) {
            occupied_seats += 1;
        }
        if self.is_occupied_seat_in_direction(x, y, 1, -1) {
            occupied_seats += 1;
        }
        if self.is_occupied_seat_in_direction(x, y, 1, 0) {
            occupied_seats += 1;
        }
        if self.is_occupied_seat_in_direction(x, y, 1, 1) {
            occupied_seats += 1;
        }
        occupied_seats
    }

    fn get_occupied_seats_at_part1(&self, x: usize, y: usize) -> u32 {
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

    fn check_position_part_2(&self, x: usize, y: usize) -> Field {
        let occupied_seats = self.get_occupied_seats_at_part2(x, y);
        match self.map[x][y] {
            Field::Empty => {
                if occupied_seats == 0 {
                    Field::Occupied
                } else {
                    Field::Empty
                }
            }
            Field::Occupied => {
                if occupied_seats >= 5 {
                    Field::Empty
                } else {
                    Field::Occupied
                }
            }
            Field::Floor => Field::Floor,
        }
    }

    fn check_position(&self, x: usize, y: usize) -> Field {
        let occupied_seats = self.get_occupied_seats_at_part1(x, y);
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

    fn count_occupied(&self) -> usize {
        self.map
            .iter()
            .map(|v| v.iter().filter(|&&x| x == Field::Occupied).count())
            .sum()
    }

    fn part_1(mut self) -> usize {
        loop {
            let done = self.step();
            if done {
                break;
            }
        }
        self.count_occupied()
    }

    fn step_part_2(&mut self) -> bool {
        let mut new_map = self.map.clone();

        for x in 0..self.map.len() {
            for y in 0..self.map[0].len() {
                new_map[x][y] = self.check_position_part_2(x, y);
            }
        }
        if self.map == new_map {
            true
        } else {
            self.map = new_map;
            false
        }
    }

    fn part_2(mut self) -> usize {
        loop {
            let done = self.step_part_2();
            if done {
                break;
            }
        }
        self.count_occupied()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let map = Map::from(contents.as_str());

    let part_1 = map.clone().part_1();
    println!("{}", part_1);

    let part_2 = map.part_2();
    println!("{}", part_2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_occupied_seats_1() {
        let input = ".......#.\n\
                     ...#.....\n\
                     .#.......\n\
                     .........\n\
                     ..#L....#\n\
                     ....#....\n\
                     .........\n\
                     #........\n\
                     ...#.....\n";
        let map = Map::from(input);
        dbg!(map.get_seat_at(4, 3));
        let occupied_seats = map.get_occupied_seats_at_part2(4, 3);
        assert_eq!(occupied_seats, 8);
    }
}
