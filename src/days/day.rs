pub trait Day<T1, T2>
where
    T1: ToString,
    T2: ToString,
{
    fn part1<I>(input: I) -> T1
    where
        I: Iterator<Item = String>;
    fn part2<I>(input: I) -> T2
    where
        I: Iterator<Item = String>;
}
