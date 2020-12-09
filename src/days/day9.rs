use crate::day::Day;

pub struct Day9;

impl Day for Day9 {
    type T1 = i64;
    fn part1<I>(input: I) -> i64
    where
        I: Iterator<Item = String>,
    {
        let mut window: [i64; 25] = [0; 25];
        let window_size = window.len();
        let mut index = 0;
        let mut values = input.map(|line| line.parse::<i64>().unwrap());

        // Prelude
        for i in 0..window_size {
            window[i] = values.next().unwrap();
        }

        loop {
            let next_value = values.next().unwrap();

            let found = window
                .iter()
                .enumerate()
                .map(|(i, x)| window.iter().skip(i + 1).map(move |y| x + y))
                .flatten()
                .any(|sum| sum == next_value);

            if found {
                window[index] = next_value;
                index += 1;
                index %= window_size;
            } else {
                return next_value;
            }
        }
    }

    type T2 = u64;
    fn part2<I>(input: I) -> u64
    where
        I: Iterator<Item = String>,
    {
        let needle = 70639851;
        let values: Vec<u64> = input.map(|line| line.parse::<u64>().unwrap()).collect();
        let mut begin = 0;
        let mut end = 0;
        let mut sum = values[begin];

        while sum != needle {
            if sum < needle {
                end += 1;
                sum += values[end];
            } else {
                sum -= values[begin];
                begin += 1;
            }
        }

        let min = values
            .iter()
            .skip(begin)
            .take(end - begin + 1)
            .min()
            .unwrap();
        let max = values
            .iter()
            .skip(begin)
            .take(end - begin + 1)
            .max()
            .unwrap();

        return min + max;
    }
}
