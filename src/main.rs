mod days;
mod util;

use days::*;
use util::file;

#[allow(dead_code)]
enum Day {
    One,
    Two,
    Three,
    Four,
}

#[allow(dead_code)]
enum Part {
    One,
    Two,
}

fn run_day<T>(day: u8, part: Part) -> String
where
    T: day::Day,
{
    let input = file::input(day);
    match part {
        Part::One => T::part1(input).to_string(),
        Part::Two => T::part2(input).to_string(),
    }
}

fn run(day: Day, part: Part) -> String {
    match day {
        Day::One => run_day::<day1::Day1>(1, part),
        Day::Two => run_day::<day2::Day2>(2, part),
        Day::Three => run_day::<day3::Day3>(3, part),
        Day::Four => run_day::<day4::Day4>(4, part),
    }
}

fn main() {
    println!("{}", run(Day::Four, Part::One));
}
