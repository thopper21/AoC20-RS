use crate::day::Day;

use std::cmp::min;
use std::iter::once;

pub struct Day10;

impl Day for Day10 {
    type T1 = usize;
    fn part1<I>(input: I) -> usize
    where
        I: Iterator<Item = String>,
    {
        let mut adapters: Vec<u64> = once(0)
            .chain(input.map(|line| line.parse::<u64>().unwrap()))
            .collect();
        adapters.sort();
        adapters.push(adapters[adapters.len() - 1] + 3);

        let (ones, threes) =
            adapters
                .iter()
                .zip(adapters.iter().skip(1))
                .fold((0, 0), |(ones, threes), (x, y)| match y - x {
                    1 => (ones + 1, threes),
                    3 => (ones, threes + 1),
                    _ => (ones, threes),
                });

        ones * threes
    }

    type T2 = usize;
    fn part2<I>(input: I) -> usize
    where
        I: Iterator<Item = String>,
    {
        let mut adapters: Vec<u64> = once(0)
            .chain(input.map(|line| line.parse::<u64>().unwrap()))
            .collect();
        adapters.sort();

        adapters
            .iter()
            .enumerate()
            .skip(1)
            .fold(vec![1, 1, 1], |back_trace, (i, adapter)| {
                let size = min(back_trace.len(), i);
                let next = back_trace
                    .iter()
                    .zip(adapters.iter().cloned().skip(i - size).take(size).rev())
                    .filter(|(_, back_adapter)| adapter - back_adapter <= 3)
                    .map(|(back, _)| back)
                    .sum();

                once(next)
                    .chain(back_trace.iter().cloned().take(back_trace.len() - 1))
                    .collect()
            })[0]
    }
}
