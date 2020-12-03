use std::fs::File;
use std::io::{self, BufRead};

pub fn lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open("input/Day2.txt")?;
    Ok(io::BufReader::new(file).lines())
}

pub fn part1<I>(_input: I) -> usize
where
    I: Iterator<Item = String>,
{
    return 42;
}

pub fn part2<I>(_input: I) -> usize
where
    I: Iterator<Item = String>,
{
    return 42;
}
