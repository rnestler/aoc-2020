use std::fs::File;
use std::io::prelude::*;

fn decode_seat(seat: &str) -> u32 {
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

    row_upper * 8 + col_upper
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let max_seat_id = contents.lines().map(decode_seat).max().unwrap();

    println!("Part 1: {}", max_seat_id);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_decode_seat() {
        let input = "FBFBBFFRLR";
        let seat_id = decode_seat(input);
        assert_eq!(seat_id, 357);
    }
}
