use gray_codes::VecSubsets;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Instruction {
    SetMask(u64, u64, u64, Vec<usize>),
    SetMemory(u64, u64),
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Instruction {
        let mut tokens = s.split('=');
        let token = tokens.next().unwrap().trim();
        match token {
            "mask" => {
                let mask = tokens.next().unwrap().trim();
                let mut bitmask_zeros = u64::MAX;
                let mut bitmask_ones = 0;
                let mut bitmask_x = 0;
                let mut bitmask_x_pos = Vec::new();
                for (index, byte) in mask.bytes().rev().enumerate() {
                    match byte {
                        b'X' => {
                            bitmask_x |= 1 << index;
                            bitmask_x_pos.push(index);
                        }
                        b'1' => {
                            bitmask_ones |= 1 << index;
                        }
                        b'0' => {
                            bitmask_zeros &= !(1 << index);
                        }
                        _ => {
                            panic!("Invalid input");
                        }
                    }
                }
                Instruction::SetMask(bitmask_zeros, bitmask_ones, bitmask_x, bitmask_x_pos)
            }
            mem => {
                let address: u64 = mem
                    .split('[')
                    .nth(1)
                    .unwrap()
                    .split(']')
                    .next()
                    .unwrap()
                    .parse()
                    .unwrap();
                let value: u64 = tokens.next().unwrap().trim().parse().unwrap();
                Instruction::SetMemory(address, value)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Machine {
    bitmask_zeros: u64,
    bitmask_ones: u64,
    bitmask_x: u64,
    bitmask_x_pos: Vec<usize>,

    memory: HashMap<u64, u64>,
}

impl Machine {
    fn new() -> Machine {
        Machine {
            bitmask_zeros: u64::MAX,
            bitmask_ones: 0,
            bitmask_x: 0,
            bitmask_x_pos: Vec::new(),
            memory: HashMap::new(),
        }
    }

    fn execute_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::SetMask(bitmask_zeros, bitmask_ones, _, _) => {
                self.bitmask_zeros = bitmask_zeros;
                self.bitmask_ones = bitmask_ones;
            }
            Instruction::SetMemory(address, value) => {
                let mem_value = self.memory.entry(address).or_insert(0);
                *mem_value = value;
                *mem_value &= self.bitmask_zeros;
                *mem_value |= self.bitmask_ones;
            }
        }
    }

    fn part_1(&mut self, instructions: &Vec<Instruction>) -> u64 {
        for instruction in instructions {
            self.execute_instruction(instruction.clone());
        }
        self.memory.values().sum()
    }

    fn execute_instruction_part_2(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::SetMask(bitmask_zeros, bitmask_ones, bitmask_x, bitmask_x_pos) => {
                self.bitmask_zeros = bitmask_zeros;
                self.bitmask_ones = bitmask_ones;
                self.bitmask_x = bitmask_x;
                self.bitmask_x_pos = bitmask_x_pos;
            }
            Instruction::SetMemory(address, value) => {
                let address = (address | self.bitmask_ones) & (!self.bitmask_x);
                for subset in VecSubsets::of(&self.bitmask_x_pos) {
                    let mut address = address;
                    for bit in subset {
                        address |= 1 << *bit;
                    }
                    *self.memory.entry(address).or_insert(0) = value;
                }
            }
        }
    }

    fn part_2(&mut self, instructions: &Vec<Instruction>) -> u64 {
        for instruction in instructions {
            self.execute_instruction_part_2(instruction.clone());
        }
        self.memory.values().sum()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let instructions: Vec<_> = contents.lines().map(Instruction::from).collect();
    let mut machine = Machine::new();
    let part_1 = machine.part_1(&instructions);
    println!("Part 1: {}", part_1);

    let mut machine = Machine::new();
    let part_2 = machine.part_2(&instructions);
    println!("Part 2: {}", part_2);

    Ok(())
}
