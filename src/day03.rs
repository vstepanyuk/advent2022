use std::{collections::HashSet, fmt::Display};

use anyhow::Result;
use aoc::{Runnable, Solution};
use aoc_derive::Runner;
use itertools::Itertools;
use rayon::prelude::*;

#[derive(Runner)]
#[aoc(file = "inputs/day03.txt")]
pub struct DaySolution {
    pub filename: &'static str,
}

trait Priority {
    fn priority(&self) -> usize;
}

impl Priority for char {
    fn priority(&self) -> usize {
        match self {
            'a'..='z' => *self as usize - b'a' as usize + 1,
            'A'..='Z' => *self as usize - b'A' as usize + 27,
            _ => unreachable!(),
        }
    }
}

impl DaySolution {}

impl Solution for DaySolution {
    fn part1(&self, input: &str) -> Result<Box<dyn Display>> {
        let result = input
            .lines()
            .par_bridge()
            .map(|line| {
                let len = line.len();

                HashSet::<char>::from_iter(line[0..len / 2].chars())
                    .intersection(&HashSet::<char>::from_iter(line[len / 2..].chars()))
                    .map(Priority::priority)
                    .sum::<usize>()
            })
            .sum::<usize>();

        Ok(Box::new(result))
    }

    fn part2(&self, input: &str) -> Result<Box<dyn Display>> {
        let result = input
            .lines()
            .tuples()
            .map(|(a, b, c)| {
                HashSet::<char>::from_iter(a.chars())
                    .intersection(&HashSet::<char>::from_iter(b.chars()))
                    .cloned()
                    .collect::<HashSet<_>>()
                    .intersection(&HashSet::<char>::from_iter(c.chars()))
                    .map(Priority::priority)
                    .sum::<usize>()
            })
            .sum::<usize>();

        Ok(Box::new(result))
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use aoc::day_test;
    use paste::paste;

    day_test!(day03, Part1, "inputs/day03_demo.txt", "157");
    day_test!(day03, Part2, "inputs/day03_demo.txt", "70");
}
