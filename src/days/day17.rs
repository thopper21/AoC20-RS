use crate::day::Day;

use std::cmp::{max, min};
use std::collections::HashSet;
use std::hash::Hash;

pub struct Day17;

#[derive(PartialEq, Eq, Hash, Clone)]
struct Coordinate3D {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Coordinate4D {
    x: i64,
    y: i64,
    z: i64,
    w: i64,
}

trait Coordinate
where
    Self: Sized,
{
    fn new(x: i64, y: i64) -> Self;
    fn min() -> Self;
    fn max() -> Self;
    fn pairwise_min(&self, other: &Self) -> Self;
    fn pairwise_max(&self, other: &Self) -> Self;
    fn range(min: &Self, max: &Self) -> Vec<Self>;
    fn adjacent(&self) -> Vec<Self>;
}

impl Coordinate for Coordinate3D {
    fn new(x:i64, y: i64) -> Self {
        Self { x, y, z: 0}
    }

    fn min() -> Self {
        Self {
            x: i64::MIN,
            y: i64::MIN,
            z: i64::MIN,
        }
    }

    fn max() -> Self {
        Self {
            x: i64::MAX,
            y: i64::MAX,
            z: i64::MAX,
        }
    }

    fn pairwise_min(&self, other: &Self) -> Self {
        Self {
            x: min(self.x, other.x),
            y: min(self.y, other.y),
            z: min(self.z, other.z),
        }
    }

    fn pairwise_max(&self, other: &Self) -> Self {
        Self {
            x: max(self.x, other.x),
            y: max(self.y, other.y),
            z: max(self.z, other.z),
        }
    }

    fn range(min: &Self, max: &Self) -> Vec<Self> {
        let mut result = Vec::<Self>::new();
        for x in (min.x - 1)..(max.x + 2) {
            for y in (min.y - 1)..(max.y + 2) {
                for z in (min.z - 1)..(max.z + 2) {
                    result.push(Self { x, y, z });
                }
            }
        }

        result
    }

    fn adjacent(&self) -> Vec<Self> {
        let mut result = Vec::<Self>::new();
        for dx in -1..2 {
            for dy in -1..2 {
                for dz in -1..2 {
                    if dx == 0 && dy == 0 && dz == 0 {
                        continue;
                    }
                    result.push(Self {
                        x: self.x + dx,
                        y: self.y + dy,
                        z: self.z + dz,
                    });
                }
            }
        }

        result
    }
}

impl Coordinate for Coordinate4D {
    fn new(x:i64, y: i64) -> Self {
        Self { x, y, z: 0, w: 0}
    }

    fn min() -> Self {
        Self {
            x: i64::MIN,
            y: i64::MIN,
            z: i64::MIN,
            w: i64::MIN,
        }
    }

    fn max() -> Self {
        Self {
            x: i64::MAX,
            y: i64::MAX,
            z: i64::MAX,
            w: i64::MAX,
        }
    }

    fn pairwise_min(&self, other: &Self) -> Self {
        Self {
            x: min(self.x, other.x),
            y: min(self.y, other.y),
            z: min(self.z, other.z),
            w: min(self.w, other.w)
        }
    }

    fn pairwise_max(&self, other: &Self) -> Self {
        Self {
            x: max(self.x, other.x),
            y: max(self.y, other.y),
            z: max(self.z, other.z),
            w: max(self.w, other.w)
        }
    }

    fn range(min: &Self, max: &Self) -> Vec<Self> {
        let mut result = Vec::<Self>::new();
        for x in (min.x - 1)..(max.x + 2) {
            for y in (min.y - 1)..(max.y + 2) {
                for z in (min.z - 1)..(max.z + 2) {
                    for w in (min.w - 1)..(max.w + 2) {
                        result.push(Self { x, y, z, w });
                    }
                }
            }
        }

        result
    }

    fn adjacent(&self) -> Vec<Self> {
        let mut result = Vec::<Self>::new();
        for dx in -1..2 {
            for dy in -1..2 {
                for dz in -1..2 {
                    for dw in -1..2 {
                        if dx == 0 && dy == 0 && dz == 0 && dw == 0 {
                            continue;
                        }
                        result.push(Self {
                            x: self.x + dx,
                            y: self.y + dy,
                            z: self.z + dz,
                            w: self.w + dw
                        });
                    }
                }
            }
        }

        result
    }
}

struct Cube<Coord> {
    active_cubes: HashSet<Coord>,
}

impl<Coord> Cube<Coord>
where
    Coord: Coordinate + Eq + Hash + Clone,
{
    fn step(&mut self) {
        let min_coord = self
            .active_cubes
            .iter()
            .fold(Coord::max(), |mins, coordinate| {
                mins.pairwise_min(coordinate)
            });
        let max_coord = self
            .active_cubes
            .iter()
            .fold(Coord::min(), |maxs, coordinate| {
                maxs.pairwise_max(coordinate)
            });
        let mut to_remove = HashSet::<Coord>::new();
        let mut to_insert = HashSet::<Coord>::new();

        for coordinate in Coord::range(&min_coord, &max_coord) {
            let adjacent = self.count_adjacent(&coordinate);
            if self.active_cubes.contains(&coordinate) {
                if adjacent != 2 && adjacent != 3 {
                    to_remove.insert(coordinate);
                }
            } else {
                if adjacent == 3 {
                    to_insert.insert(coordinate);
                }
            }
        }

        self.active_cubes = self.active_cubes.difference(&to_remove).cloned().collect();
        self.active_cubes = self.active_cubes.union(&to_insert).cloned().collect();
    }

    fn count_adjacent(&self, coordinate: &Coord) -> usize {
        let mut count = 0;
        for adjacent in coordinate.adjacent() {
            if self.active_cubes.contains(&adjacent) {
                count += 1;
            }
        }

        count
    }
}

fn test_input<Coord>() -> Cube<Coord> where Coord: Coordinate + Eq + Hash {
    let mut active_cubes = HashSet::<Coord>::new();
    active_cubes.insert(Coord::new(1, 0));
    active_cubes.insert(Coord::new(2, 1));
    active_cubes.insert(Coord::new(0, 2));
    active_cubes.insert(Coord::new(1, 2));
    active_cubes.insert(Coord::new(2, 2));

    Cube::<Coord> { active_cubes }
}

fn parse<I, Coord>(input: I) -> Cube<Coord>
where
    I: Iterator<Item = String>,
    Coord: Coordinate + Hash + Eq
{
    let mut active_cubes = HashSet::<Coord>::new();
    for (y, line) in input.enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                active_cubes.insert(Coord::new(x as i64, y as i64));
            }
        }
    }

    Cube::<Coord> { active_cubes }
}

fn run<I, Coord>(input: I) -> usize
where I: Iterator<Item = String>, Coord: Coordinate + Eq + Hash + Clone {
    let mut cube = parse::<I, Coord>(input);
    // let mut cube = test_input::<Coord>();
    for _ in 0..6 {
        cube.step();
    }

    cube.active_cubes.len()
}

impl Day for Day17 {
    type T1 = usize;
    fn part1<I>(input: I) -> usize
    where
        I: Iterator<Item = String>,
    {
        run::<I, Coordinate3D>(input)
    }

    type T2 = usize;
    fn part2<I>(input: I) -> usize
    where
        I: Iterator<Item = String>,
    {
        run::<I, Coordinate4D>(input)
    }
}
