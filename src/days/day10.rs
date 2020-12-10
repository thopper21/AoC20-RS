use crate::day::Day;

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

        let mut back_trace = [1, 1, 1];

        for i in 1..adapters.len() {
            let mut next = 0;
            for j in 0..back_trace.len() {
                if i >= j + 1 && adapters[i] - adapters[i - j - 1] <= 3 {
                    next += back_trace[j];
                }
            }

            for j in (1..back_trace.len()).rev() {
                back_trace[j] = back_trace[j - 1];
            }
            back_trace[0] = next;
        }

        back_trace[0]
    }
}
