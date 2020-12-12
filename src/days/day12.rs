use crate::day::Day;

pub struct Day12;

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

struct Position {
    x: i64,
    y: i64,
    dir: Direction,
    dx: i64,
    dy: i64,
}

enum Op {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

struct Instruction {
    op: Op,
    magnitude: i64,
}

// Rotate clockwise
fn rotate(start: Direction, deg: i64) -> Direction {
    [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ]
    .iter()
    .cycle()
    .skip_while(|dir| **dir != start)
    .nth((deg / 90) as usize)
    .unwrap()
    .clone()
}

// Rotate a point clockwise
fn rotate_about((x, y): (i64, i64), deg: i64) -> (i64, i64) {
    [(x, y), (y, -x), (-x, -y), (-y, x)]
        .iter()
        .cycle()
        .nth((deg / 90) as usize)
        .unwrap()
        .clone()
}

impl Position {
    fn new() -> Position {
        Position {
            x: 0,
            y: 0,
            dir: Direction::East,
            dx: 10,
            dy: 1,
        }
    }

    fn manhattan(&self) -> i64 {
        self.x.abs() + self.y.abs()
    }

    fn step_ship(&mut self, instruction: Instruction) {
        match instruction.op {
            Op::North => {
                self.y += instruction.magnitude;
            }
            Op::South => {
                self.y -= instruction.magnitude;
            }
            Op::East => {
                self.x += instruction.magnitude;
            }
            Op::West => {
                self.x -= instruction.magnitude;
            }
            Op::Left => {
                self.dir = rotate(self.dir, 360 - instruction.magnitude);
            }
            Op::Right => {
                self.dir = rotate(self.dir, instruction.magnitude);
            }
            Op::Forward => {
                let op = match self.dir {
                    Direction::North => Op::North,
                    Direction::South => Op::South,
                    Direction::East => Op::East,
                    Direction::West => Op::West,
                };
                self.step_ship(Instruction {
                    op,
                    magnitude: instruction.magnitude,
                });
            }
        }
    }

    fn step_waypoint(&mut self, instruction: Instruction) {
        match instruction.op {
            Op::North => {
                self.dy += instruction.magnitude;
            }
            Op::South => {
                self.dy -= instruction.magnitude;
            }
            Op::East => {
                self.dx += instruction.magnitude;
            }
            Op::West => {
                self.dx -= instruction.magnitude;
            }
            Op::Left => {
                let (dx, dy) = rotate_about((self.dx, self.dy), 360 - instruction.magnitude);
                self.dx = dx;
                self.dy = dy;
            }
            Op::Right => {
                let (dx, dy) = rotate_about((self.dx, self.dy), instruction.magnitude);
                self.dx = dx;
                self.dy = dy;
            }
            Op::Forward => {
                self.x += self.dx * instruction.magnitude;
                self.y += self.dy * instruction.magnitude;
            }
        }
    }
}

fn parse(line: String) -> Instruction {
    let op_code = line.chars().next().unwrap();
    let op = match op_code {
        'N' => Op::North,
        'S' => Op::South,
        'E' => Op::East,
        'W' => Op::West,
        'L' => Op::Left,
        'R' => Op::Right,
        'F' => Op::Forward,
        _ => panic!("Unable to parse: {}", line),
    };
    let magnitude = line[1..].parse::<i64>().unwrap();

    Instruction { op, magnitude }
}

impl Day for Day12 {
    type T1 = i64;
    fn part1<I>(input: I) -> i64
    where
        I: Iterator<Item = String>,
    {
        let mut pos = Position::new();
        for instruction in input.map(parse) {
            pos.step_ship(instruction);
        }

        pos.manhattan()
    }

    type T2 = i64;
    fn part2<I>(input: I) -> i64
    where
        I: Iterator<Item = String>,
    {
        let mut pos = Position::new();
        for instruction in input.map(parse) {
            pos.step_waypoint(instruction);
        }

        pos.manhattan()
    }
}
