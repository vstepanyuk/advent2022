use std::{
    fmt::Display,
    ops::{Deref, RangeInclusive},
    str::FromStr,
};

use anyhow::Result;
use aoc::{Runnable, Solution};
use aoc_derive::Runner;

#[derive(Runner)]
#[aoc(file = "inputs/day04.txt")]
pub struct DaySolution {
    pub filename: &'static str,
}

#[derive(Debug)]
struct Section {
    inner: RangeInclusive<usize>,
}

impl Deref for Section {
    type Target = RangeInclusive<usize>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl FromStr for Section {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s
            .split_once('-')
            .ok_or_else(|| anyhow::anyhow!("Invalid range"))?;

        let start = start.parse()?;
        let end = end.parse()?;

        Ok(Self { inner: start..=end })
    }
}

impl DaySolution {
    fn solve<F>(&self, input: &str, comparison: F) -> usize
    where
        F: Fn(bool, bool) -> bool,
    {
        input
            .lines()
            .map(|line| {
                let (first, second) = line.split_once(',').unwrap();
                (
                    first.parse::<Section>().unwrap(),
                    second.parse::<Section>().unwrap(),
                )
            })
            .filter(|(r1, r2)| {
                comparison(r1.contains(r2.start()), r1.contains(r2.end()))
                    || comparison(r2.contains(r1.start()), r2.contains(r1.end()))
            })
            .count()
    }
}

impl Solution for DaySolution {
    fn part1(&self, input: &str) -> Result<Box<dyn Display>> {
        Ok(Box::new(self.solve(input, |a, b| a && b)))
    }

    fn part2(&self, input: &str) -> Result<Box<dyn Display>> {
        Ok(Box::new(self.solve(input, |a, b| a || b)))
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use aoc::day_test;
    use paste::paste;

    day_test!(day04, Part1, "inputs/day04_demo.txt", "2");
    day_test!(day04, Part2, "inputs/day04_demo.txt", "4");
}
