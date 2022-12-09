use std::fmt::Display;

use anyhow::Result;
use aoc::{Runnable, Solution};
use aoc_derive::Runner;

use crate::matrix::{Matrix, MATRIX_NEIGHBOURS_4};

#[derive(Runner)]
#[aoc(file = "inputs/day08_demo.txt")]
pub struct DaySolution {
    pub filename: &'static str,
}

impl DaySolution {}

impl Solution for DaySolution {
    fn part1(&self, input: &str) -> Result<Box<dyn Display>> {
        let count = Matrix::<u8>::from(input)
            .unwrap()
            .iter_with_self()
            .filter(|&(v, (x, y), matrix)| {
                (x == 0 || y == 0 || x == matrix.width - 1 || y == matrix.height - 1)
                    || (0..x).all(|x| matrix.get(x, y).unwrap() < v)
                    || (x + 1..matrix.width).all(|x| matrix.get(x, y).unwrap() < v)
                    || (0..y).all(|y| matrix.get(x, y).unwrap() < v)
                    || (y + 1..matrix.height).all(|y| matrix.get(x, y).unwrap() < v)
            })
            .count();

        Ok(Box::new(count))
    }

    fn part2(&self, input: &str) -> Result<Box<dyn Display>> {
        let max_scenic = Matrix::<u8>::from(input).unwrap().iter_with_self().fold(
            0,
            |max_scenic, (v, (x, y), matrix)| {
                MATRIX_NEIGHBOURS_4
                    .iter()
                    .fold(1, |result, (dx, dy)| {
                        let mut nx = x as i32;
                        let mut ny = y as i32;
                        let mut count = 0;

                        while let Some(value) = matrix.get(nx + dx, ny + dy) {
                            count += 1;

                            nx += dx;
                            ny += dy;

                            if value >= v {
                                break;
                            }
                        }
                        result * count
                    })
                    .max(max_scenic)
            },
        );

        Ok(Box::new(max_scenic))
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use aoc::day_test;
    use paste::paste;

    day_test!(day08, Part1, "inputs/day08_demo.txt", "21");
    day_test!(day08, Part2, "inputs/day08_demo.txt", "8");
}
