use crate::day::Day;

pub struct Day5;

impl Day for Day5 {
    type T1 = u64;
    fn part1<I>(input: I) -> u64
    where
        I: Iterator<Item = String>,
    {
        input.count() as u64
    }

    type T2 = u64;
    fn part2<I>(input: I) -> u64
    where
        I: Iterator<Item = String>,
    {
        input.count() as u64
    }
}
