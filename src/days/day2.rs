extern crate nom;
extern crate regex;

use self::nom::{
    bytes::complete::tag,
    character::complete::{alpha1, anychar, digit1},
    map, named, IResult,
};
use std::num::ParseIntError;
use std::str::FromStr;

struct PasswordConstraint {
    min: usize,
    max: usize,
    letter: char,
}

struct Input {
    password: String,
    constraint: PasswordConstraint,
}

named!(size<&str, Result<usize, ParseIntError>>,
    map!(digit1, FromStr::from_str)
);

fn input_parser(input: &str) -> IResult<&str, Input> {
    let (input, min_result) = size(input)?;
    let (input, _) = tag("-")(input)?;
    let (input, max_result) = size(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, letter) = anychar(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, password_str) = alpha1(input)?;

    let min = min_result.unwrap();
    let max = max_result.unwrap();
    let password = String::from(password_str);
    return IResult::Ok((
        input,
        Input {
            constraint: PasswordConstraint { min, max, letter },
            password,
        },
    ));
}

fn parse_input(input: String) -> Input {
    let (_, result) = input_parser(&input[..]).unwrap();

    return result;
}

fn validate_password1(input: &Input) -> bool {
    let password = &input.password;
    let constraint = &input.constraint;
    let counter_letters = password.chars().filter(|&c| c == constraint.letter).count();
    return constraint.min <= counter_letters && counter_letters <= constraint.max;
}

fn validate_password2(input: &Input) -> bool {
    let password = &input.password;
    let constraint = &input.constraint;

    let contains = |i| password.chars().nth(i - 1).unwrap() == constraint.letter;

    return contains(constraint.min) ^ contains(constraint.max);
}

fn run<I, F>(input: I, validate: F) -> usize
where
    I: Iterator<Item = String>,
    F: Fn(&Input) -> bool,
{
    return input.map(parse_input).filter(validate).count();
}

pub fn part1<I>(input: I) -> usize
where
    I: Iterator<Item = String>,
{
    return run(input, &validate_password1);
}

pub fn part2<I>(input: I) -> usize
where
    I: Iterator<Item = String>,
{
    return run(input, &validate_password2);
}
