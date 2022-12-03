use std::{collections::HashSet, fmt::Display};

use anyhow::Result;
use aoc::{Runnable, Solution};
use aoc_derive::Runner;

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
            .split_whitespace()
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
            .split_whitespace()
            .collect::<Vec<_>>()
            .chunks(3)
            .map(|w| {
                w.iter()
                    .map(|line| HashSet::<char>::from_iter(line.chars()))
                    .enumerate()
                    .fold(HashSet::new(), |acc, (i, set)| {
                        if i == 0 {
                            set
                        } else {
                            acc.intersection(&set).cloned().collect()
                        }
                    })
                    .iter()
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
