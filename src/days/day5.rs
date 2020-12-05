use crate::day::Day;

use std::collections::HashSet;

pub struct Day5;

struct BinarySpace {
    min: u64,
    max: u64,
}

impl BinarySpace {
    fn new(size: u32) -> BinarySpace {
        BinarySpace { min: 0, max: 2u64.pow(size) - 1 }
    }

    fn lower(&mut self) {
        self.max -= (self.max - self.min + 1) / 2;
    }

    fn upper(&mut self) {
        self.min += (self.max - self.min + 1) / 2;
    }

    fn value(&self) -> u64 {
        assert!(self.max == self.min);
        self.max
    }
}

fn to_seat_id(line: String) -> u64 {
    let mut row_space = BinarySpace::new(7);
    let mut col_space = BinarySpace::new(3);

    for c in line.chars() {
        match c {
            'F' => row_space.lower(),
            'B' => row_space.upper(),
            'R' => col_space.upper(),
            'L' => col_space.lower(),
            _ => panic!("Unexpected character"),
        }
    }

    row_space.value() * 8 + col_space.value()
}

impl Day for Day5 {
    type T1 = u64;
    fn part1<I>(input: I) -> u64
    where
        I: Iterator<Item = String>,
    {
        input.map(to_seat_id).max().unwrap()
    }

    type T2 = u64;
    fn part2<I>(input: I) -> u64
    where
        I: Iterator<Item = String>,
    {
        let seat_ids: HashSet<u64> = input.map(to_seat_id).collect();

        // Infinite range but we just want the first one satisfying these conditions
        (1..)
            .find(|i| {
                !seat_ids.contains(&i) && seat_ids.contains(&(i - 1)) && seat_ids.contains(&(i + 1))
            })
            .unwrap()
    }
}
