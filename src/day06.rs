use std::{collections::HashSet, fmt::Display};

use anyhow::Result;
use aoc::{Runnable, Solution};
use aoc_derive::Runner;

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
            .find_map(|(pos, chars)| {
                if HashSet::<u8>::from_iter(chars.iter().cloned()).len() == size {
                    Some(pos + size)
                } else {
                    None
                }
            })
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
