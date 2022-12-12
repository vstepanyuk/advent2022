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
    fn find_path<SF, WF>(
        &self,
        matrix: &mut Matrix<char>,
        start: (char, usize, usize),
        end: (char, usize, usize),
        distance: WF,
        success: SF,
    ) -> Result<usize>
    where
        SF: Fn(&(char, usize, usize)) -> bool,
        WF: Fn(&char, &char) -> i32,
    {
        *matrix.get_mut(start.1, start.2).unwrap() = start.0;
        *matrix.get_mut(end.1, end.2).unwrap() = end.0;

        let result = bfs(
            &start,
            |&(current, x, y)| {
                matrix
                    .neighbours4_iter(x, y)
                    .filter_map(|(&c, (x, y))| {
                        (distance(&c, &current) <= 1).then_some((c, x as usize, y as usize))
                    })
                    .collect::<Vec<_>>()
            },
            success,
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
            .find_map(|(c, (x, y))| (*c == 'S').then_some(('a', x, y)))
            .ok_or_else(|| anyhow::anyhow!("No start"))?;

        let end = matrix
            .iter()
            .find_map(|(c, (x, y))| (*c == 'E').then_some(('z', x, y)))
            .ok_or_else(|| anyhow::anyhow!("No end"))?;

        Ok(Box::new(self.find_path(
            &mut matrix,
            start,
            end,
            |n, c| *n as i32 - *c as i32,
            |p| p.1 == end.1 && p.2 == end.2,
        )?))
    }

    fn part2(&self, input: &str) -> Result<Box<dyn Display>> {
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

        Ok(Box::new(self.find_path(
            &mut matrix,
            end,
            start,
            |n, c| *c as i32 - *n as i32,
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
