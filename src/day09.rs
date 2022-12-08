use std::fmt::Display;

use anyhow::Result;
use aoc::{Runnable, Solution};
use aoc_derive::Runner;

#[derive(Runner)]
#[aoc(file = "inputs/day09.txt")]
pub struct DaySolution {
    pub filename: &'static str,
}

impl DaySolution {}

impl Solution for DaySolution {
    fn part1(&self, _input: &str) -> Result<Box<dyn Display>> {
        Ok(Box::new(0))
    }

    fn part2(&self, _input: &str) -> Result<Box<dyn Display>> {
        Ok(Box::new(0))
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use aoc::day_test;
    use paste::paste;

    day_test!(day09, Part1, "inputs/day09_demo.txt", "unknown");
    day_test!(day09, Part2, "inputs/day09_demo.txt", "unknown");
}
