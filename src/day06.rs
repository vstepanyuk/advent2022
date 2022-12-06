use std::fmt::Display;

use anyhow::Result;
use aoc::{Runnable, Solution};
use aoc_derive::Runner;
use itertools::Itertools;

#[derive(Runner)]
#[aoc(file = "inputs/day06.txt")]
pub struct DaySolution {
    pub filename: &'static str,
}

impl DaySolution {
    fn solve(&self, input: &str, size: usize) -> Option<usize> {
        input
            .as_bytes()
            .windows(size)
            .enumerate()
            .find_map(|(pos, chars)| chars.iter().all_unique().then_some(pos + size))
    }
}

impl Solution for DaySolution {
    fn part1(&self, input: &str) -> Result<Box<dyn Display>> {
        Ok(Box::new(self.solve(input, 4).unwrap()))
    }

    fn part2(&self, input: &str) -> Result<Box<dyn Display>> {
        Ok(Box::new(self.solve(input, 14).unwrap()))
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use aoc::day_test;
    use paste::paste;

    day_test!(day06, Part1, "inputs/day06_demo.txt", "7");
    day_test!(day06, Part2, "inputs/day06_demo.txt", "19");
}
