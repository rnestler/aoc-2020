use std::fs::File;
use std::io::prelude::*;
use std::collections::{HashMap, HashSet};

use lazy_static::lazy_static;
use regex::Regex;

// map_part_1 tells which bag can be placed in which bags
// map_part_2 tells which bag contains which bags
fn parse_line(input: &str, map_part_1: &mut HashMap<String, Vec<String>>, map_part_2: &mut HashMap<String, Vec<(usize, String)>>) {
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
            let count = captures.get(1).unwrap().as_str().parse().unwrap();
            map_part_1.entry(bag.into()).or_insert(vec![]).push(container_bag.into());
            map_part_2.entry(container_bag.into()).or_insert(vec![]).push((count, bag.into()));
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

fn part_2(map: &HashMap<String, Vec<(usize, String)>>, search: &str) -> usize {
    let mut count = 0;
    if let Some(next) = map.get(search) {
        for search in next.iter() {
            count += (part_2(map, &search.1) + 1) * search.0;
        }
    }
    count
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    let mut map_part_1 = HashMap::new();
    let mut map_part_2 = HashMap::new();
    for line in contents.lines() {
        parse_line(line, &mut map_part_1, &mut map_part_2);
    }

    let mut set = HashSet::new();
    part_1(&map_part_1, &mut set, "shiny gold");
    println!("Part 1: {}", set.len());

    let count = part_2(&map_part_2, "shiny gold");
    println!("Part 2: {}", count);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
}
