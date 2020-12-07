use std::fs::File;
use std::io::prelude::*;
use std::collections::{HashMap, HashSet};

use lazy_static::lazy_static;
use regex::Regex;

// map tells which bag can be placed in which bags
fn parse_line(input: &str, map: &mut HashMap<String, Vec<String>>) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^([a-z]+ [a-z]+) bags? contain (.*)$").unwrap();
        static ref RE_CONTENT: Regex = Regex::new(r"([0-9]+) ([a-z]+ [a-z]+) bags?").unwrap();
    }
    let captures = RE.captures(input).unwrap();
    let container_bag = captures.get(1).unwrap().as_str();
    let content = captures.get(2).unwrap().as_str();
    content.split(',').for_each(|v| {
        let captures = RE_CONTENT.captures(v);
        if let Some(captures) = captures {
            let bag = captures.get(2).unwrap().as_str();
            map.entry(bag.into()).or_insert(vec![]).push(container_bag.into());
        }
    });
}

fn part_1(map: &HashMap<String, Vec<String>>, output: &mut HashSet<String>, search: &str) {
    if let Some(next) = map.get(search) {

        output.extend(next.clone());
        for search in next.iter() {
            part_1(map, output, search);
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    let mut map = HashMap::new();
    for line in contents.lines() {
        parse_line(line, &mut map);
    }

    let mut set = HashSet::new();
    part_1(&map, &mut set, "shiny gold");
    println!("Part 1: {}", set.len());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
}
