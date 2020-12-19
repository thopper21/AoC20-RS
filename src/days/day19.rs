use crate::day::Day;

pub struct Day19;

impl Day for Day19 {
    type T1 = usize;
    fn part1<I>(input: I) -> usize
    where
        I: Iterator<Item = String>,
    {
        input.count()
    }

    type T2 = usize;
    fn part2<I>(input: I) -> usize
    where
        I: Iterator<Item = String>,
    {
        input.count()
    }
}
