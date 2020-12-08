use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let op = parts.next().ok_or("err")?;
        let arg: i32 = parts
            .next()
            .ok_or("err")?
            .parse()
            .map_err(|_| "failed to parse int")?;
        match op {
            "nop" => Ok(Instruction::Nop(arg)),
            "acc" => Ok(Instruction::Acc(arg)),
            "jmp" => Ok(Instruction::Jmp(arg)),
            _ => Err("Not implemented".into()),
        }
    }
}

struct Machine {
    accumulator: i32,
    pc: usize,
    code: Vec<Instruction>,
}

impl Machine {
    pub fn new(code: Vec<Instruction>) -> Machine {
        Machine {
            accumulator: 0,
            pc: 0,
            code,
        }
    }

    pub fn step(&mut self) -> usize {
        match self.code[self.pc] {
            Instruction::Nop(_) => {
                self.pc += 1;
            }
            Instruction::Acc(acc) => {
                self.accumulator += acc;
                self.pc += 1;
            }
            Instruction::Jmp(offset) => {
                if offset > 0 {
                    self.pc += offset as usize;
                } else {
                    self.pc -= (-offset) as usize;
                }
            }
        }
        self.pc
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let code = contents
        .lines()
        .enumerate()
        .map(|(n, line)| {
            Instruction::from_str(line).expect(&format!("Failed to parse line {}: {}", n, line))
        })
        .collect();
    let mut machine = Machine::new(code);

    let mut visited = vec![0usize];
    loop {
        let pc = machine.step();
        if visited.contains(&pc) {
            println!("Part 1: {}", machine.accumulator);
            break;
        }
        visited.push(pc);
    }

    Ok(())
}
