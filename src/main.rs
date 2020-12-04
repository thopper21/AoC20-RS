mod days;
mod util;

use days::*;
use util::file;

#[allow(dead_code)]
enum Day {
    One,
    Two,
    Three,
}

#[allow(dead_code)]
enum Part {
    One,
    Two,
}

fn run_day<T, T1, T2>(day: u8, part: Part) -> String
where
    T: day::Day<T1, T2>,
    T1: ToString,
    T2: ToString,
{
    let input = file::input(day);
    match part {
        Part::One => T::part1(input).to_string(),
        Part::Two => T::part2(input).to_string(),
    }
}

fn run(day: Day, part: Part) -> String {
    match day {
        Day::One => run_day::<day1::Day1, i32, i32>(1, part),
        Day::Two => run_day::<day2::Day2, usize, usize>(2, part),
        Day::Three => run_day::<day3::Day3, usize, usize>(3, part),
    }
}

fn main() {
    println!("{}", run(Day::Three, Part::Two));
}
