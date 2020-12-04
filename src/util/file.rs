use std::fs::File;
use std::io::{self, BufRead};

pub fn input(day: u8) -> impl Iterator<Item = String> {
    let file = File::open(format!("input/Day{}.txt", day)).unwrap();
    io::BufReader::new(file).lines().map(|line| line.unwrap())
}
