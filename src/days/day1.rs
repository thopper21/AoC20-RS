use crate::day::Day;

pub struct Day1;

impl Day for Day1 {
    type T1 = i32;
    fn part1<I>(input: I) -> i32
    where
        I: Iterator<Item = String>,
    {
        let values: std::collections::HashSet<i32> =
            input.map(|line| line.parse::<i32>().unwrap()).collect();
        for value in &values {
            let target = 2020 - value;
            if values.contains(&target) {
                return value * target;
            }
        }
        return 0;
    }

    type T2 = i32;
    fn part2<I>(input: I) -> i32
    where
        I: Iterator<Item = String>,
    {
        let values: std::collections::HashSet<i32> =
            input.map(|line| line.parse::<i32>().unwrap()).collect();
        for outer in &values {
            let outer_target = 2020 - outer;
            for inner in &values {
                let inner_target = outer_target - inner;
                if values.contains(&inner_target) {
                    return outer * inner * inner_target;
                }
            }
        }

        return 0;
    }
}
