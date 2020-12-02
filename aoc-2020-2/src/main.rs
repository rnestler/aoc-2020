use std::fs::File;
use std::io::prelude::*;

struct Range {
    pub min: usize,
    pub max: usize,
}

struct Rule {
    pub range: Range,
    pub letter: u8,
    pub string: String,
}

impl From<&str> for Range {
    fn from(s: &str) -> Self {
        let mut tokens = s.split('-').map(|t| t.parse::<usize>().unwrap());
        Range {
            min: tokens.next().unwrap(),
            max: tokens.next().unwrap(),
        }
    }
}

impl From<&str> for Rule {
    fn from(s: &str) -> Self {
        let mut tokens = s.split_whitespace();
        let range = Range::from(tokens.next().unwrap());
        let letter = tokens.next().unwrap().bytes().nth(0).unwrap();
        let string = tokens.next().unwrap().to_string();
        Rule {
            range,
            letter,
            string,
        }
    }
}

impl Rule {
    pub fn is_valid(&self) -> bool {
        let count = self.string.bytes().filter(|c| *c == self.letter).count();
        count >= self.range.min && count <= self.range.max
    }
}

fn part1(input: &str) -> usize {
    input.lines().map(Rule::from).filter(|rule| rule.is_valid()).count()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let result_part1 = part1(&contents);
    println!("{}", result_part1);

    Ok(())
}
