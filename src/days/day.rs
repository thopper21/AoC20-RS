pub trait Day {
    type T1: ToString;
    fn part1<I>(input: I) -> Self::T1
    where
        I: Iterator<Item = String>;

    type T2: ToString;
    fn part2<I>(input: I) -> Self::T2
    where
        I: Iterator<Item = String>;
}
