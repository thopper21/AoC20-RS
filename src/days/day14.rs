use crate::day::Day;

use std::collections::HashMap;

#[derive(Clone, Copy)]
enum TriState {
    Unset,
    Zero,
    One,
}

#[derive(Clone)]
struct Mask {
    bits: [TriState; 36],
}

impl Mask {
    fn apply(&self, mut val: u64) -> u64 {
        for i in 0..36 {
            match self.bits[i] {
                TriState::One => val |= 1 << i,
                TriState::Zero => val &= !(1 << i),
                _ => continue,
            }
        }

        val
    }

    fn parse(string: &str) -> Mask {
        let mut bits = [TriState::Unset; 36];
        let mut chars = string.chars();

        for i in 0..36 {
            match &chars.next().unwrap() {
                '1' => bits[35 - i] = TriState::One,
                '0' => bits[35 - i] = TriState::Zero,
                _ => continue,
            }
        }

        Mask { bits }
    }
}

enum Instruction {
    Mask(Mask),
    Mem(u64, u64),
}

struct Program {
    mask: Mask,
    memory: HashMap<u64, u64>,
}

impl Program {
    fn new() -> Program {
        Program {
            mask: Mask {
                bits: [TriState::Unset; 36],
            },
            memory: HashMap::<u64, u64>::new(),
        }
    }

    fn apply(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Mask(mask) => {
                self.mask = mask.clone();
            }
            Instruction::Mem(address, value) => {
                self.memory.insert(*address, self.mask.apply(*value));
            }
        }
    }
}

fn parse(line: String) -> Instruction {
    if line.starts_with("mask") {
        Instruction::Mask(Mask::parse(&line[7..]))
    } else {
        let open_bracket = line.find('[').unwrap();
        let close_bracket = line.find(']').unwrap();
        let equals = line.find('=').unwrap();

        Instruction::Mem(
            line[(open_bracket+1)..close_bracket].parse::<u64>().unwrap(),
            line[(equals + 2)..].parse::<u64>().unwrap()
        )
    }
}

pub struct Day14;

impl Day for Day14 {
    type T1 = u64;
    fn part1<I>(input: I) -> u64
    where
        I: Iterator<Item = String>,
    {
        // let instructions = vec![
        //     Instruction::Mask(Mask::parse(
        //         "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string(),
        //     )),
        //     Instruction::Mem(8, 11),
        //     Instruction::Mem(7, 101),
        //     Instruction::Mem(8, 0),
        // ];
        let instructions = input.map(parse);

        let mut program = Program::new();

        for instruction in instructions {
            program.apply(&instruction);
        }

        program.memory.values().sum()
    }

    type T2 = usize;
    fn part2<I>(mut input: I) -> usize
    where
        I: Iterator<Item = String>,
    {
        input.count()
    }
}
