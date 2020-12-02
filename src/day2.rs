extern crate regex;

use self::regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};

struct PasswordConstraint {
    min: usize,
    max: usize,
    letter: char,
}

struct Input {
    password: String,
    constraint: PasswordConstraint,
}

fn parse_input(line: String) -> Input {
    // {min}-{max} {letter}: password
    let re = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)$").unwrap();
    let captures = re.captures_iter(&line).next().unwrap();

    let min = captures[1].parse::<usize>().unwrap();
    let max = captures[2].parse::<usize>().unwrap();
    let letter = captures[3].chars().next().unwrap();
    let password = captures[4].to_string();

    let constraint = PasswordConstraint { min, max, letter };
    return Input {
        constraint,
        password,
    };
}

fn validate_password(input: &Input) -> bool {
    let password = &input.password;
    let constraint = &input.constraint;
    let counter_letters = password.chars().filter(|&c| c == constraint.letter).count();
    return constraint.min <= counter_letters && counter_letters <= constraint.max;
}

pub fn lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open("input/Day2.txt")?;
    Ok(io::BufReader::new(file).lines())
}

pub fn part1<I>(input: I) -> usize
where
    I: Iterator<Item = String>,
{
    return input.map(parse_input).filter(validate_password).count();
}
