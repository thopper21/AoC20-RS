use crate::day::Day;

pub struct Day11;

#[derive(PartialEq, Eq, Clone)]
enum Cell {
    Floor,
    Empty,
    Occupied,
}

struct Ferry {
    cells: Vec<Vec<Cell>>,
}

impl Ferry {
    fn update(&mut self, changes: &Vec<(usize, usize)>) {
        for &(i, j) in changes {
            self.flip(i, j);
        }
    }

    fn flip(&mut self, row: usize, col: usize) {
        match &self.cells[row][col] {
            Cell::Empty => self.cells[row][col] = Cell::Occupied,
            Cell::Occupied => self.cells[row][col] = Cell::Empty,
            _ => panic!("Only seats can flip!"),
        }
    }

    fn get_adjacent_occupied(&self, row: usize, col: usize) -> usize {
        let dirs = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        let mut count = 0;

        let is_valid = |i, j| {
            if i < 0 || j < 0 {
                return false;
            }

            if i as usize >= self.cells.len() {
                return false;
            }

            let row = &self.cells[i as usize];
            if j as usize >= row.len() {
                return false;
            }

            return true;
        };

        for (dx, dy) in dirs.iter() {
            let i = (row as i64) + dx;
            let j = (col as i64) + dy;

            if is_valid(i, j) {
                match self.cells[i as usize][j as usize] {
                    Cell::Occupied => {
                        count += 1;
                    }
                    _ => {}
                }
            }
        }

        count
    }

    fn get_visible_occupied(&self, row: usize, col: usize) -> usize {
        let dirs = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        let mut count = 0;

        let is_valid = |i, j| {
            if i < 0 || j < 0 {
                return false;
            }

            if i as usize >= self.cells.len() {
                return false;
            }

            let row = &self.cells[i as usize];
            if j as usize >= row.len() {
                return false;
            }

            return true;
        };

        for (dx, dy) in dirs.iter() {
            let mut i = (row as i64) + dx;
            let mut j = (col as i64) + dy;

            while is_valid(i, j) {
                match self.cells[i as usize][j as usize] {
                    Cell::Empty => {
                        break;
                    }
                    Cell::Occupied => {
                        count += 1;
                        break;
                    }
                    _ => {}
                }

                i += dx;
                j += dy;
            }
        }

        count
    }
}

fn parse(line: String) -> Vec<Cell> {
    line.chars()
        .map(|c| match c {
            '.' => Cell::Floor,
            'L' => Cell::Empty,
            _ => panic!("Unexpected input: {}", line),
        })
        .collect()
}

fn run<I, F>(input: I, threshold: usize, count_adj: F) -> usize
where
    I: Iterator<Item = String>,
    F: Fn(&Ferry, usize, usize) -> usize,
{
    let mut ferry = Ferry {
        cells: input.map(parse).collect(),
    };

    loop {
        let mut changes = Vec::<(usize, usize)>::new();

        for i in 0..ferry.cells.len() {
            let row = &ferry.cells[i];
            for j in 0..row.len() {
                let cell = &row[j];
                let adj = count_adj(&ferry, i, j);
                match cell {
                    Cell::Empty => {
                        if adj == 0 {
                            changes.push((i, j));
                        }
                    }
                    Cell::Occupied => {
                        if adj >= threshold {
                            changes.push((i, j));
                        }
                    }
                    _ => {}
                }
            }
        }

        if changes.is_empty() {
            return ferry
                .cells
                .iter()
                .map(|row| row.iter().filter(|cell| **cell == Cell::Occupied).count())
                .sum();
        } else {
            ferry.update(&changes);
        }
    }
}

impl Day for Day11 {
    type T1 = usize;
    fn part1<I>(input: I) -> usize
    where
        I: Iterator<Item = String>,
    {
        run(input, 4, |ferry, i, j| ferry.get_adjacent_occupied(i, j))
    }

    type T2 = usize;
    fn part2<I>(input: I) -> usize
    where
        I: Iterator<Item = String>,
    {
        run(input, 5, |ferry, i, j| ferry.get_visible_occupied(i, j))
    }
}
