mod days;

use days::{*};

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
            let input = day1::lines().unwrap().map(|line| line.unwrap());
            let result = match part {
                Part::One => day1::part1(input),
                Part::Two => day1::part2(input),
            };
            return result.unwrap().to_string();
        }
        Day::Two => {
            let input = day2::lines().unwrap().map(|line| line.unwrap());
            let result = match part {
                Part::One => day2::part1(input),
                Part::Two => day2::part2(input),
            };
            return result.to_string();
        }
        Day::Three => {
            let input = day3::lines().unwrap().map(|line| line.unwrap());
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
