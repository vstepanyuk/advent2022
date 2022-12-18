use std::{collections::HashSet, fmt::Display, str::FromStr};

use anyhow::Result;
use aoc::{Runnable, Solution};
use aoc_derive::Runner;
use itertools::{process_results, Itertools};
use pathfinding::prelude::bfs;

#[derive(Runner)]
#[aoc(file = "inputs/day18.txt")]
pub struct DaySolution {
    pub filename: &'static str,
}

impl DaySolution {
    fn parse(&self, input: &str) -> Result<Vec<Cube1>> {
        process_results(input.lines().map(str::parse), |it| it.collect_vec())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Cube1 {
    x: i32,
    y: i32,
    z: i32,
}

impl Cube1 {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Cube1 { x, y, z }
    }

    fn connected(&self, other: &Self) -> bool {
        if self.x == other.x && self.y == other.y && self.z == other.z + 1 {
            return true;
        }

        if self.x == other.x && self.y == other.y && self.z == other.z - 1 {
            return true;
        }

        if self.x == other.x && self.z == other.z && self.y == other.y + 1 {
            return true;
        }

        if self.x == other.x && self.z == other.z && self.y == other.y - 1 {
            return true;
        }

        if self.z == other.z && self.y == other.y && self.x == other.x + 1 {
            return true;
        }

        if self.z == other.z && self.y == other.y && self.x == other.x - 1 {
            return true;
        }

        false
    }
}

impl FromStr for Cube1 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(',');
        let x = iter.next().unwrap().parse::<i32>()?;
        let y = iter.next().unwrap().parse::<i32>()?;
        let z = iter.next().unwrap().parse::<i32>()?;

        Ok(Cube1::new(x, y, z))
    }
}

impl Solution for DaySolution {
    fn part1(&self, input: &str) -> Result<Box<dyn Display>> {
        let cubes = self.parse(input)?;
        let result = itertools::iproduct!(cubes.iter(), cubes.iter()).fold(
            cubes.len() * 6,
            |acc, (c1, c2)| {
                if c1 != c2 && c1.connected(c2) {
                    acc - 1
                } else {
                    acc
                }
            },
        );

        Ok(Box::new(result))
    }

    fn part2(&self, input: &str) -> Result<Box<dyn Display>> {
        let cubes: HashSet<Cube1> = HashSet::from_iter(self.parse(input)?.into_iter());

        let (min_x, max_x) = cubes.iter().map(|c| c.x).minmax().into_option().unwrap();
        let (min_y, max_y) = cubes.iter().map(|c| c.y).minmax().into_option().unwrap();
        let (min_z, max_z) = cubes.iter().map(|c| c.z).minmax().into_option().unwrap();

        let mut result = 0;
        _ = bfs(
            &Cube1::new(min_x - 1, min_y - 1, min_z - 1),
            |&cube| {
                result += cubes.iter().filter(|c| c.connected(&cube)).count();

                let mut neighbours = vec![];
                for offset in [-1, 1] {
                    let x_cube = Cube1::new(cube.x + offset, cube.y, cube.z);
                    let y_cube = Cube1::new(cube.x, cube.y + offset, cube.z);
                    let z_cube = Cube1::new(cube.x, cube.y, cube.z + offset);

                    if !cubes.contains(&x_cube) && x_cube.x >= min_x - 1 && x_cube.x <= max_x + 1 {
                        neighbours.push(x_cube);
                    }

                    if !cubes.contains(&y_cube) && y_cube.y >= min_y - 1 && y_cube.y <= max_y + 1 {
                        neighbours.push(y_cube);
                    }

                    if !cubes.contains(&z_cube) && z_cube.z >= min_z - 1 && z_cube.z <= max_z + 1 {
                        neighbours.push(z_cube);
                    }
                }

                neighbours
            },
            |&cube| cube == Cube1::new(max_x + 1, max_y + 1, max_z + 1),
        );

        Ok(Box::new(result))
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use aoc::day_test;
    use paste::paste;

    day_test!(day18, Part1, "inputs/day18_demo.txt", "64");
    day_test!(day18, Part2, "inputs/day18_demo.txt", "58");
}
