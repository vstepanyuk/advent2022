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

impl DaySolution {
    fn find_path(
        &self,
        matrix: &mut Matrix<char>,
        start: (usize, usize),
        end: (usize, usize),
    ) -> Result<usize> {
        *matrix.get_mut(start.0, start.1).unwrap() = 'a';
        *matrix.get_mut(end.0, end.1).unwrap() = 'z';

        let result = bfs(
            &start,
            |&(x, y)| {
                let current = *matrix.get(x, y).unwrap();
                matrix
                    .neighbours4_iter(x, y)
                    .filter_map(|(&c, (x, y))| {
                        (c as i32 - current as i32 <= 1).then_some((x as usize, y as usize))
                    })
                    .collect::<Vec<_>>()
            },
            |p| p == &end,
        );

        Ok(result
            .ok_or_else(|| anyhow::anyhow!("No path found"))
            .map(|path| path.len() - 1)?)
    }
}

impl Solution for DaySolution {
    fn part1(&self, input: &str) -> Result<Box<dyn Display>> {
        let mut matrix =
            Matrix::<char>::from(input).ok_or_else(|| anyhow::anyhow!("Invalid input"))?;

        let start = matrix
            .iter()
            .find_map(|(c, (x, y))| (*c == 'S').then_some((x, y)))
            .ok_or_else(|| anyhow::anyhow!("No start"))?;

        let end = matrix
            .iter()
            .find_map(|(c, (x, y))| (*c == 'E').then_some((x, y)))
            .ok_or_else(|| anyhow::anyhow!("No end"))?;

        Ok(Box::new(self.find_path(&mut matrix, start, end)?))
    }

    fn part2(&self, input: &str) -> Result<Box<dyn Display>> {
        let matrix = Matrix::<char>::from(input).ok_or_else(|| anyhow::anyhow!("Invalid input"))?;

        let end = matrix
            .iter()
            .find_map(|(c, (x, y))| (*c == 'E').then_some((x, y)))
            .ok_or_else(|| anyhow::anyhow!("No end"))?;

        let result = matrix
            .iter()
            .filter_map(|(c, (x, y))| (*c == 'S' || *c == 'a').then_some((x, y)))
            .filter_map(|start| self.find_path(&mut matrix.clone(), start, end).ok())
            .min()
            .unwrap();

        Ok(Box::new(result))
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
