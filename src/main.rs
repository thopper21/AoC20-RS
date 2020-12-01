use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("input/Day1.txt") {
        let values: Vec<i32> = lines.map(|line| line.unwrap().parse::<i32>().unwrap()).collect();
        for x in &values {
            for y in &values {
                if x + y == 2020 {
                    println!("{}", x * y);
                    return;
                }
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
