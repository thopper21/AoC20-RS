use crate::day::Day;

use std::collections::HashSet;
use std::iter::once;

pub struct Day6;

impl Day for Day6 {
    type T1 = usize;
    fn part1<I>(input: I) -> usize
    where
        I: Iterator<Item = String>,
    {
        let mut qs = HashSet::<char>::new();
        let mut result = 0;

        for line in input {
            if line.len() == 0 {
                result += qs.len();
                qs = HashSet::<char>::new();
            }

            for c in line.chars() {
                qs.insert(c);
            }
        }

        result += qs.len();

        result
    }

    type T2 = usize;
    fn part2<I>(input: I) -> usize
    where
        I: Iterator<Item = String>,
    {
        // 'a'..'z' doesn't include 'z' :(
        let init: HashSet<char> = ('a'..'z').chain(once('z')).collect();

        let mut qs = init.clone();
        let mut result = 0;

        for line in input {
            if line.len() == 0 {
                result += qs.len();
                qs = init.clone();
            } else {
                qs = qs
                    .intersection(&(line.chars().collect()))
                    .cloned()
                    .collect();
            }
        }

        result += qs.len();

        result
    }
}
