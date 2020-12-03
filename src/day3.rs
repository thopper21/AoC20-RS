use std::fs::File;
use std::io::{self, BufRead};

pub fn lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open("input/Day3.txt")?;
    Ok(io::BufReader::new(file).lines())
}

pub fn part1<I>(input: I) -> usize
where
    I: Iterator<Item = String>,
{
    let dx = 3;
    let mut x = 0;
    let mut trees = 0;

    for line in input {
        let len = line.chars().count();
        if x >= len {
            x %= len;
        }

        if line.chars().nth(x) == Some('#') {
            trees += 1;
        }

        x += dx;
    }

    return trees;
}

pub fn part2<I>(input: I) -> usize
where
    I: Iterator<Item = String>,
{
    let dxs = [1, 3, 5, 7, 1];
    let dys = [1, 1, 1, 1, 2];
    let mut xs = [0, 0, 0, 0, 0];
    let mut trees = [0, 0, 0, 0, 0];
    let mut y = 0;

    for line in input {
        let len = line.chars().count();
        for i in 0..dxs.len() {
            if y % dys[i] != 0 {
                continue;
            }

            if xs[i] >= len {
                xs[i] %= len;
            }

            if line.chars().nth(xs[i]) == Some('#') {
                trees[i] += 1;
            }

            xs[i] += dxs[i];
        }

        y += 1;
    }

    let mut result = 1;

    for i in 0..trees.len() {
        result *= trees[i];
        if trees[i] == 0 {
            println!("No trees for {}", i);
        }
    }

    return result;
}
