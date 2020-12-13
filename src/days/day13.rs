use crate::day::Day;

use std::iter::successors;

pub struct Day13;

impl Day for Day13 {
    type T1 = u64;
    fn part1<I>(mut input: I) -> u64
    where
        I: Iterator<Item = String>,
    {
        let time = input.next().unwrap().parse::<u64>().unwrap();
        let bus_list = input.next().unwrap();
        let buses = bus_list
            .split(',')
            .map(|s| s.parse::<u64>())
            .filter(|x| x.is_ok())
            .map(|x| x.unwrap());

        let mut wait_time = u64::MAX;
        let mut bus_id = u64::MAX;

        for bus in buses {
            let bus_wait_time = (time / bus + 1) * bus - time;
            if bus_wait_time < wait_time {
                wait_time = bus_wait_time;
                bus_id = bus;
            }
        }

        wait_time * bus_id
    }

    type T2 = u64;
    fn part2<I>(mut input: I) -> u64
    where
        I: Iterator<Item = String>,
    {
        let _ = input.next();
        let bus_list = input.next().unwrap();
        let buses: Vec<(u64, u64)> = bus_list
            .split(',')
            .map(|s| s.parse::<u64>())
            .enumerate()
            .filter(|(_, x)| x.is_ok())
            .map(|(i, x)| {
                let modulo = x.unwrap();
                let offset = i as u64;
                ((modulo*offset - offset) % modulo, modulo)
            })
            .collect();

        // Sieve approach to Chinese remainder theorem
        buses
            .iter()
            .fold((0, 1), |(value, addend), (target, modulo)| {
                (
                    successors(Some(value), |curr| Some(curr + addend))
                        .filter(|value| value % modulo == *target)
                        .next()
                        .unwrap(),
                    addend * modulo,
                )
            })
            .0
    }
}
