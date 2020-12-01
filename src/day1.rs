use std::fs::File;
use std::io::{self, BufRead};

pub fn lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open("input/Day1.txt")?;
    Ok(io::BufReader::new(file).lines())
}

pub fn part1<I>(input: I) -> Option<i32>
where
    I: Iterator<Item = String>,
{
    let values: Vec<i32> = input.map(|line| line.parse::<i32>().unwrap()).collect();
    for x in &values {
        for y in &values {
            if x + y == 2020 {
                return Some(x * y);
            }
        }
    }
    return None;
}

pub fn part2<I>(input: I) -> Option<i32>
where
    I: Iterator<Item = String>,
{
    let values: Vec<i32> = input.map(|line| line.parse::<i32>().unwrap()).collect();
    for x in &values {
        for y in &values {
            for z in &values {
                if x + y + z == 2020 {
                    return Some(x * y * z);
                }
            }
        }
    }
    return None;
}