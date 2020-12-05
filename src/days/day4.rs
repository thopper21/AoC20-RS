use crate::day::Day;
use std::collections::HashMap;
use std::collections::HashSet;

pub struct Day4;

enum Line {
    PartialPassport(Vec<(String, String)>),
    EndOfPassport,
}

fn parse_line(line: String) -> Line {
    if line.is_empty() {
        return Line::EndOfPassport;
    }

    return Line::PartialPassport(
        line.split(" ")
            .map(|pair| {
                let parts: Vec<&str> = pair.split(":").collect();
                (parts[0].to_string(), parts[1].to_string())
            })
            .collect(),
    );
}

fn is_complete(pairs: &HashMap<String, String>) -> bool {
    let expected = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    expected
        .iter()
        .all(|key| pairs.contains_key(&key.to_string()))
}

fn is_valid(pairs: &HashMap<String, String>) -> bool {
    let validate =
        |key, pred: &dyn Fn(&String) -> bool| -> bool { pairs.get(key).map_or(false, pred) };

    let in_range =
        |min, max| move |s: &String| s.parse::<i32>().map_or(false, |x| min <= x && x <= max);

    // byr (Birth Year) - four digits; at least 1920 and at most 2002.
    validate("byr", &in_range(1920, 2002)) && 
    // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    validate("iyr", &in_range(2010, 2020)) &&
    // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    validate("eyr", &in_range(2020, 2030)) &&
    // hgt (Height) - a number followed by either cm or in:
    // If cm, the number must be at least 150 and at most 193.
    // If in, the number must be at least 59 and at most 76.
    validate("hgt", &|hgt| {
        if hgt.ends_with("cm") {
            in_range(150, 193)(&hgt[0..hgt.len()-2].to_string())
        } else if hgt.ends_with("in") {
            in_range(59, 76)(&hgt[0..hgt.len()-2].to_string())
        } else {
            false
        }
    }) &&
    // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    validate("hcl", &|hcl| {
        hcl.starts_with("#") && (hcl[1..].chars().all(|c|
            c.is_numeric() || (c >= 'a' && c <= 'f')))
    }) &&
    // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    validate("ecl", &|ecl| {
        let expected: HashSet<String> = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].iter().map(|s| s.to_string()).collect();
        expected.contains(ecl)
    }) &&
    // pid (Passport ID) - a nine-digit number, including leading zeroes.
    validate("pid", &|pid| {
        pid.len() == 9 && pid.chars().all(|c| c.is_numeric())
    })
    // cid (Country ID) - ignored, missing or not.
}

fn run<I, F>(input: I, pred: F) -> u64
where
    I: Iterator<Item = String>,
    F: Fn(&HashMap<String, String>) -> bool,
{
    let lines = input.map(parse_line);
    let mut keys = HashMap::<String, String>::new();
    let mut result = 0;

    for line in lines {
        match line {
            Line::EndOfPassport => {
                if pred(&keys) {
                    result += 1;
                }
                keys.clear();
            }
            Line::PartialPassport(pairs) => {
                for (key, value) in pairs {
                    keys.insert(key, value);
                }
            }
        }
    }

    if is_complete(&keys) {
        result += 1;
    }

    return result;
}

impl Day for Day4 {
    type T1 = u64;
    fn part1<I>(input: I) -> u64
    where
        I: Iterator<Item = String>,
    {
        run(input, is_complete)
    }

    type T2 = u64;
    fn part2<I>(input: I) -> u64
    where
        I: Iterator<Item = String>,
    {
        run(input, is_valid)
    }
}
