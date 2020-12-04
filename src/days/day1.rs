pub fn part1<I>(input: I) -> Option<i32>
where
    I: Iterator<Item = String>,
{
    let values: std::collections::HashSet<i32> =
        input.map(|line| line.parse::<i32>().unwrap()).collect();
    for value in &values {
        let target = 2020 - value;
        if values.contains(&target) {
            return Some(value * target);
        }
    }
    return None;
}

pub fn part2<I>(input: I) -> Option<i32>
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
                return Some(outer * inner * inner_target);
            }
        }
    }

    return None;
}
