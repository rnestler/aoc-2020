use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::num::ParseIntError;
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

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

impl FieldContent {
    fn is_valid(&self) -> bool {
        match self {
            FieldContent::BirthYear(byr) => *byr >= 1920 && *byr <= 2002,
            FieldContent::IssueYear(iyr) => *iyr >= 2010 && *iyr <= 2020,
            FieldContent::ExpirationYear(eyr) => *eyr >= 2020 && *eyr <= 2030,
            FieldContent::Height(hgt) => {
                lazy_static! {
                    static ref RE_IN: Regex = Regex::new(r"^([[:digit:]]{2})in$").unwrap();
                    static ref RE_CM: Regex = Regex::new(r"^([[:digit:]]{3})cm$").unwrap();
                }
                if RE_CM.is_match(hgt) {
                    let cm = RE_CM.captures(hgt).unwrap().get(1).unwrap();
                    let cm = cm.as_str().parse::<u32>().unwrap();
                    cm >= 150 && cm <= 193
                } else if RE_IN.is_match(hgt) {
                    let inch = RE_IN.captures(hgt).unwrap().get(1).unwrap();
                    let inch = inch.as_str().parse::<u32>().unwrap();
                    inch >= 59 && inch <= 76
                } else {
                    false
                }
            }
            FieldContent::HairColor(hcl) => {
                lazy_static! {
                    static ref RE: Regex = Regex::new(r"^#[0-9,a-f]{6}$").unwrap();
                }
                RE.is_match(hcl)
            }
            FieldContent::EyeColor(ecl) => match ecl.as_str() {
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                _ => false,
            },
            FieldContent::PassportID(pid) => {
                lazy_static! {
                    static ref RE: Regex = Regex::new(r"^[[:digit:]]{9}$").unwrap();
                }
                RE.is_match(pid)
            }
            FieldContent::CountryID(_) => true,
        }
    }
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

    fn is_valid_part_2(&self) -> bool {
        if !self.is_valid() {
            return false;
        }
        self.fields.values().all(|value| value.is_valid())
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

fn part_2(passports: &Vec<Passport>) -> u32 {
    let mut cnt = 0;
    for passport in passports {
        if passport.is_valid_part_2() {
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

    println!("Part 2: {}", part_2(&passports));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fields_from_str() {
        let input = "ecl:gry";
        assert_eq!(
            Field::from_str(input),
            Ok(Field("ecl".into(), FieldContent::EyeColor("gry".into())))
        );
    }

    #[test]
    fn test_hcl_valid() {
        let input = "hcl:#602927";
        let field = Field::from_str(input).unwrap();
        assert!(field.1.is_valid());
    }

    #[test]
    fn test_hgt_valid() {
        let input = "hgt:183cm";
        let field = Field::from_str(input).unwrap();
        assert!(field.1.is_valid());
    }

    #[test]
    fn test_invalid_passports_part_2() {
        let inputs = [
            "eyr:1972 cid:100 hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926",
            "iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946",
            "hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277",
            "hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007",
        ];
        for input in &inputs {
            let passport = Passport::from_str(input).unwrap();
            assert!(!passport.is_valid_part_2());
        }
    }
}
