use std::fs::File;
use std::io::prelude::*;

fn decode_seat(seat: &str) -> (u32, u32) {
    let mut row_upper = 127;
    let mut row_lower = 0;
    let mut col_upper = 7;
    let mut col_lower = 0;

    for byte in seat.bytes() {
        match byte {
            b'F' => {
                row_upper = (row_lower + row_upper) / 2;
            }
            b'B' => {
                row_lower = (row_lower + row_upper + 1) / 2;
            }

            b'L' => {
                col_upper = (col_lower + col_upper) / 2;
            }
            b'R' => {
                col_lower = (col_lower + col_upper + 1) / 2;
            }
            _ => unreachable!(),
        }
    }
    (row_upper, col_upper)
}

fn get_seat_id(row: u32, col: u32) -> u32 {
    row * 8 + col
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut seat_ids: Vec<_> = contents
        .lines()
        .map(decode_seat)
        .map(|(r, c)| get_seat_id(r, c))
        .collect();

    let max_seat_id = seat_ids.iter().max().unwrap();
    println!("Part 1: {}", max_seat_id);

    seat_ids.sort();

    for (i, seat) in seat_ids.iter().enumerate() {
        if seat_ids[i + 1] - seat == 2 {
            println!("Part 2: {}", seat + 1);
            break;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_decode_seat() {
        let input = "FBFBBFFRLR";
        let (row, col) = decode_seat(input);
        let seat_id = get_seat_id(row, col);
        assert_eq!(seat_id, 357);
    }
}
