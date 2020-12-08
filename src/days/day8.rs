use crate::day::Day;

use std::collections::HashSet;

pub struct Day8;

enum Op {
    Nop(i64),
    Acc(i64),
    Jmp(i64),
}

struct Program {
    instructions: Vec<Op>,
    ip: i64,
    accumulator: i64,
}

enum TerminalStatus {
    Loops(i64),
    Terminates(i64),
}

impl Program {
    fn new(instructions: Vec<Op>) -> Program {
        Program {
            instructions: instructions,
            ip: 0,
            accumulator: 0,
        }
    }

    fn step(&mut self) {
        let current = &self.instructions[self.ip as usize];
        match current {
            Op::Nop(_) => {
                self.ip += 1;
            }
            Op::Acc(acc) => {
                self.accumulator += acc;
                self.ip += 1;
            }
            Op::Jmp(jmp) => {
                self.ip += jmp;
            }
        }
    }

    fn reset(&mut self) {
        self.ip = 0;
        self.accumulator = 0;
    }

    fn run(&mut self) -> TerminalStatus {
        let mut instructions_seen = HashSet::<i64>::new();
        loop {
            if self.ip >= self.instructions.len() as i64 {
                return TerminalStatus::Terminates(self.accumulator);
            }

            instructions_seen.insert(self.ip);
            self.step();
            if instructions_seen.contains(&self.ip) {
                return TerminalStatus::Loops(self.accumulator);
            }
        }
    }

    fn invert_op(&mut self, ip: usize) {
        let new_op = match self.instructions[ip] {
            Op::Nop(value) => Op::Jmp(value),
            Op::Acc(value) => Op::Acc(value),
            Op::Jmp(value) => Op::Nop(value),
        };

        self.instructions[ip] = new_op;
    }
}

fn parse_arg(arg: &str) -> i64 {
    let sign = arg.chars().next().unwrap();
    let value = arg[1..].parse::<i64>().unwrap();

    match sign {
        '+' => value,
        '-' => -value,
        _ => panic!("Cannot parse arg: {}", arg),
    }
}

fn parse_op(line: String) -> Op {
    let mut parts = line.split_whitespace();
    let op = parts.next().unwrap();
    let arg = parts.next().unwrap();

    match op {
        "nop" => Op::Nop(parse_arg(arg)),
        "acc" => Op::Acc(parse_arg(arg)),
        "jmp" => Op::Jmp(parse_arg(arg)),
        _ => panic!("Unexpected op: {}", op),
    }
}

impl Day for Day8 {
    type T1 = i64;
    fn part1<I>(input: I) -> i64
    where
        I: Iterator<Item = String>,
    {
        let mut program = Program::new(input.map(parse_op).collect());
        if let TerminalStatus::Loops(result) = program.run() {
            return result;
        }

        panic!("Program does not loop!");
    }

    type T2 = i64;
    fn part2<I>(input: I) -> i64
    where
        I: Iterator<Item = String>,
    {
        let mut program = Program::new(input.map(parse_op).collect());

        for i in 0..program.instructions.len() {
            program.invert_op(i);

            if let TerminalStatus::Terminates(result) = program.run() {
                return result;
            }

            program.invert_op(i);
            program.reset();
        }

        panic!("Could not find terminal state");
    }
}
