use std::fmt::Display;

use anyhow::Result;
use aoc::{Runnable, Solution};
use aoc_derive::Runner;

use pathfinding::prelude::{bfs, Matrix};

#[derive(Runner)]
#[aoc(file = "inputs/day12.txt")]
pub struct DaySolution {
    pub filename: &'static str,
}

type Coord = (usize, usize);

impl DaySolution {
    fn parse(&self, input: &str) -> Result<(Matrix<u8>, Coord, Coord)> {
        let mut matrix = Matrix::from_rows(input.lines().map(|l| l.as_bytes().to_vec()))?;

        let start = matrix
            .indices()
            .find(|&pos| matrix.get(pos).unwrap().eq(&b'S'))
            .ok_or_else(|| anyhow::anyhow!("No start"))?;

        let end = matrix
            .indices()
            .find(|&pos| matrix.get(pos).unwrap().eq(&b'E'))
            .ok_or_else(|| anyhow::anyhow!("No end"))?;

        *matrix.get_mut(start).unwrap() = b'a';
        *matrix.get_mut(end).unwrap() = b'z';

        Ok((matrix, start, end))
    }

    fn path_len<T, U, V>(
        &self,
        matrix: &Matrix<T>,
        start: Coord,
        mut can_move: U,
        success: V,
    ) -> Result<usize>
    where
        U: FnMut(&Coord, &Coord) -> bool,
        V: Fn(&Coord) -> bool,
    {
        bfs(
            &start,
            |&pos| {
                matrix
                    .neighbours(pos, false)
                    .filter(|neighbour| can_move(&pos, neighbour))
                    .collect::<Vec<_>>()
            },
            success,
        )
        .ok_or_else(|| anyhow::anyhow!("No path found"))
        .map(|path| path.len() - 1)
    }
}

impl Solution for DaySolution {
    fn part1(&self, input: &str) -> Result<Box<dyn Display>> {
        let (matrix, start, end) = self.parse(input)?;

        let result = self.path_len(
            &matrix,
            start,
            |&current, &neighbour| {
                let current = *matrix.get(current).unwrap() as i32;
                let neighbour = *matrix.get(neighbour).unwrap() as i32;

                neighbour - current <= 1
            },
            |&pos| pos == end,
        )?;

        Ok(Box::new(result))
    }

    fn part2(&self, input: &str) -> Result<Box<dyn Display>> {
        let (matrix, _, end) = self.parse(input)?;
        let result = self.path_len(
            &matrix,
            end,
            |&current, &neighbour| {
                let current = *matrix.get(current).unwrap() as i32;
                let neighbour = *matrix.get(neighbour).unwrap() as i32;

                current - neighbour <= 1
            },
            |&pos| *matrix.get(pos).unwrap() == b'a',
        )?;

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
