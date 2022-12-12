use std::fmt::Display;

use anyhow::Result;
use aoc::{Runnable, Solution};
use aoc_derive::Runner;

use crate::matrix::Matrix;
use pathfinding::prelude::bfs;

#[derive(Runner)]
#[aoc(file = "inputs/day12.txt")]
pub struct DaySolution {
    pub filename: &'static str,
}

type Key = (char, usize, usize);

impl DaySolution {
    fn parse(&self, input: &str) -> Result<(Matrix<char>, Key, Key)> {
        let mut matrix =
            Matrix::<char>::from(input).ok_or_else(|| anyhow::anyhow!("Invalid input"))?;

        let start = matrix
            .iter()
            .find_map(|(c, (x, y))| (*c == 'S').then_some(('a', x, y)))
            .ok_or_else(|| anyhow::anyhow!("No start"))?;

        let end = matrix
            .iter()
            .find_map(|(c, (x, y))| (*c == 'E').then_some(('z', x, y)))
            .ok_or_else(|| anyhow::anyhow!("No end"))?;

        *matrix.get_mut(start.1, start.2).unwrap() = start.0;
        *matrix.get_mut(end.1, end.2).unwrap() = end.0;

        Ok((matrix, start, end))
    }

    fn find_path<T, U>(
        &self,
        matrix: &Matrix<char>,
        start: (char, usize, usize),
        distance: U,
        success: T,
    ) -> Result<usize>
    where
        T: Fn(&(char, usize, usize)) -> bool,
        U: Fn(char, char) -> i32,
    {
        let result = bfs(
            &start,
            |&(current, x, y)| {
                matrix
                    .neighbours4_iter(x, y)
                    .filter_map(|(&neighbour, (x, y))| {
                        (distance(neighbour, current) <= 1)
                            .then_some((neighbour, x as usize, y as usize))
                    })
                    .collect::<Vec<_>>()
            },
            success,
        );

        result
            .ok_or_else(|| anyhow::anyhow!("No path found"))
            .map(|path| path.len() - 1)
    }
}

impl Solution for DaySolution {
    fn part1(&self, input: &str) -> Result<Box<dyn Display>> {
        let (matrix, start, end) = self.parse(input)?;

        Ok(Box::new(self.find_path(
            &matrix,
            start,
            |n, c| n as i32 - c as i32,
            |&(_, x, y)| x == end.1 && y == end.2,
        )?))
    }

    fn part2(&self, input: &str) -> Result<Box<dyn Display>> {
        let (matrix, _, end) = self.parse(input)?;

        Ok(Box::new(self.find_path(
            &matrix,
            end,
            |n, c| c as i32 - n as i32,
            |&(c, _, _)| c == 'a',
        )?))
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use aoc::day_test;
    use paste::paste;

    day_test!(day12, Part1, "inputs/day12_demo.txt", "31");
    day_test!(day12, Part2, "inputs/day12_demo.txt", "29");
}
