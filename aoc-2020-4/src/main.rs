use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
enum FieldContent {
    BirthYear(u32),
    IssueYear(u32),
    ExpirationYear(u32),
    Height(String),
    HairColor(String),
    EyeColor(String),
    // pid can also contain non numeric values
    PassportID(String),
    CountryID(u32),
}

#[derive(Debug, PartialEq, Eq)]
struct Field(String, FieldContent);

impl FromStr for Field {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut field = s.split(':');
        let key = field.next().unwrap();
        let value = field.next().unwrap();
        let content = match key {
            "byr" => FieldContent::BirthYear(value.parse()?),
            "iyr" => FieldContent::IssueYear(value.parse()?),
            "eyr" => FieldContent::ExpirationYear(value.parse()?),
            "hgt" => FieldContent::Height(value.into()),
            "hcl" => FieldContent::HairColor(value.into()),
            "ecl" => FieldContent::EyeColor(value.into()),
            "pid" => FieldContent::PassportID(value.into()),
            "cid" => FieldContent::CountryID(value.parse()?),
            _ => panic!("unknown field"),
        };
        Ok(Field(key.into(), content))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Passport {
    fields: HashMap<String, FieldContent>,
}

impl FromStr for Passport {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut fields = HashMap::<String, FieldContent>::new();

        for field in s.split_whitespace().map(|field| Field::from_str(field)) {
            let field = field?;
            fields.insert(field.0, field.1);
        }

        Ok(Passport { fields })
    }
}

impl Passport {
    fn is_valid(&self) -> bool {
        self.fields.contains_key("byr")
            && self.fields.contains_key("iyr")
            && self.fields.contains_key("eyr")
            && self.fields.contains_key("hgt")
            && self.fields.contains_key("hcl")
            && self.fields.contains_key("ecl")
            && self.fields.contains_key("pid")
    }
}

fn part_1(passports: &Vec<Passport>) -> u32 {
    let mut cnt = 0;
    for passport in passports {
        if passport.is_valid() {
            cnt += 1;
        }
    }
    cnt
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let passports: Vec<Passport> = contents
        .split("\n\n")
        .map(|p| Passport::from_str(p).unwrap())
        .collect();
    println!("Part 1: {}", part_1(&passports));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fields_from_str() {
        let input = "ecl:gry";
        assert_eq!(Field::from_str(input), Ok(Field::EyeColor("gry".into())));
    }
}
