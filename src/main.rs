mod day1;

#[allow(dead_code)]
enum Day {
    One,
    Two,
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
        },
        Day::Two => {
            return String::new();
        }
    }
}

fn main() {
    println!("{}", run(Day::One, Part::Two));
}
