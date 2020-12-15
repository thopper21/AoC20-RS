use crate::day::Day;

use std::collections::HashMap;

pub struct Day15;

fn nth<I>(mut input: I, n: usize) -> usize
where
    I: Iterator<Item = String>,
{
    let line = input.next().unwrap();
    let start: Vec<usize> = line
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let mut last_seen = HashMap::<usize, usize>::new();
    let mut prev = 0;

    for i in 0..n {
        if i < start.len() {
            prev = start[i];
            last_seen.insert(prev, i + 1);
        } else {
            prev = match last_seen.insert(prev, i) {
                Some(prev_i) => i - prev_i,
                None => 0,
            }
        }
    }

    prev
}

impl Day for Day15 {
    type T1 = usize;
    fn part1<I>(input: I) -> usize
    where
        I: Iterator<Item = String>,
    {
        nth(input, 2020)
    }

    type T2 = usize;
    fn part2<I>(input: I) -> usize
    where
        I: Iterator<Item = String>,
    {
        nth(input, 30000000)
    }
}
