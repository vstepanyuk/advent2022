use std::fmt::Display;

use anyhow::Result;
use aoc::{Runnable, Solution};
use aoc_derive::Runner;

#[derive(Runner)]
#[aoc(file = "inputs/day01.txt")]
pub struct DaySolution {
    pub filename: &'static str,
}

impl DaySolution {
    fn callories(&self, input: &str) -> Vec<i32> {
        input
            .split("\n\n")
            .map(|calories| {
                calories
                    .split('\n')
                    .fold(0, |acc, calories| acc + calories.parse::<i32>().unwrap())
            })
            .collect()
    }
}

impl Solution for DaySolution {
    fn part1(&self, input: &str) -> Result<Box<dyn Display>> {
        let result = *self.callories(input).iter().max().unwrap();
        Ok(Box::new(result))
    }

    fn part2(&self, input: &str) -> Result<Box<dyn Display>> {
        let mut result = self.callories(input);
        result.sort();

        Ok(Box::new(result.iter().rev().take(3).sum::<i32>()))
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use aoc::day_test;
    use paste::paste;

    day_test!(day01, Part1, "inputs/day01_demo.txt", "24000");
    day_test!(day01, Part2, "inputs/day01_demo.txt", "45000");
}
