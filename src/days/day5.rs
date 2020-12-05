use crate::day::Day;

use std::collections::HashSet;

pub struct Day5;

fn to_seat_id(line: String) -> u64 {
    let mut id = 0;
    for c in line.chars() {
        let value = match c {
            'F' => 0,
            'B' => 1,
            'R' => 1,
            'L' => 0,
            _ => panic!("Unexpected character"),
        };
        id = 2*id + value;
    }

    id
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
