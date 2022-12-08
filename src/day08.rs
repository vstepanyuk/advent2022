use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
};

use anyhow::Result;
use aoc::{Runnable, Solution};
use aoc_derive::Runner;

use crate::matrix::{Matrix, MATRIX_NEIGHBOURS_4};

#[derive(Runner)]
#[aoc(file = "inputs/day08.txt")]
pub struct DaySolution {
    pub filename: &'static str,
}

impl DaySolution {}

impl Solution for DaySolution {
    fn part1(&self, input: &str) -> Result<Box<dyn Display>> {
        let matrix: Matrix<u8> = Matrix::from(input).unwrap();
        let count = matrix
            .iter()
            .filter(|&(v, (x, y))| {
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
        let matrix: Matrix<u8> = Matrix::from(input).unwrap();

        let mut max_scenic = 0;

        matrix.iter().for_each(|(v, (x, y))| {
            let mut result = 1;

            for (dx, dy) in MATRIX_NEIGHBOURS_4.iter() {
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
                result *= count;
            }

            max_scenic = max_scenic.max(result);
        });

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
