mod days;
mod util;

use days::{*};
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

fn run(day: Day, part: Part) -> String {
    match day {
        Day::One => {
            let input = file::input(1);
            let result = match part {
                Part::One => day1::part1(input),
                Part::Two => day1::part2(input),
            };
            return result.unwrap().to_string();
        }
        Day::Two => {
            let input = file::input(2);
            let result = match part {
                Part::One => day2::part1(input),
                Part::Two => day2::part2(input),
            };
            return result.to_string();
        }
        Day::Three => {
            let input = file::input(3);
            let result = match part {
                Part::One => day3::part1(input),
                Part::Two => day3::part2(input),
            };
            return result.to_string();
        }
    }
}

fn main() {
    println!("{}", run(Day::Three, Part::Two));
}
