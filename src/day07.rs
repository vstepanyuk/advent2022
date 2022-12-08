use std::{fmt::Display, str::FromStr};

use anyhow::Result;
use aoc::{Runnable, Solution};
use aoc_derive::Runner;

#[derive(Runner)]
#[aoc(file = "inputs/day07.txt")]
pub struct DaySolution {
    pub filename: &'static str,
}

#[derive(Debug)]
enum ShellLogEntry {
    ListCommand,
    ChangeDirCommand(String),
    Directory(String),
    File(String, usize),
}

impl FromStr for ShellLogEntry {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_whitespace().collect::<Vec<_>>();

        if parts.len() < 2 {
            return Err(anyhow::anyhow!("Invalid log entry: {}", s));
        }

        match (parts[0], parts[1]) {
            ("$", "ls") => Ok(Self::ListCommand),
            ("$", "cd") => Ok(Self::ChangeDirCommand(parts[2].to_string())),
            ("dir", dir) => Ok(Self::Directory(dir.to_string())),
            (_, file) => {
                let size = parts[0].parse::<usize>()?;
                Ok(Self::File(file.to_string(), size))
            }
        }
    }
}

impl DaySolution {
    fn dir_sizes(&self, entries: impl Iterator<Item = ShellLogEntry>) -> Vec<usize> {
        let mut stack = vec![];
        let mut sizes = vec![];

        for entry in entries {
            match entry {
                ShellLogEntry::ChangeDirCommand(dir) => {
                    if dir == ".." {
                        let value = stack.pop().unwrap();
                        sizes.push(value);

                        _ = stack.last_mut().map(|v| *v += value);
                    } else {
                        stack.push(0);
                    }
                }
                ShellLogEntry::File(_, size) => {
                    _ = stack.last_mut().map(|v| *v += size);
                }
                _ => {}
            }
        }

        while let Some(size) = stack.pop() {
            sizes.push(size);
            _ = stack.last_mut().map(|v| *v += size);
        }

        sizes
    }

    fn solve<F>(&self, input: &str, mut solver: F) -> Result<usize>
    where
        F: FnMut(&[usize]) -> Result<usize>,
    {
        let log = input
            .lines()
            .filter_map(|l| l.parse::<ShellLogEntry>().ok());

        solver(&self.dir_sizes(log))
    }
}

impl Solution for DaySolution {
    fn part1(&self, input: &str) -> Result<Box<dyn Display>> {
        let result = self
            .solve(input, |sizes| {
                Ok(sizes.iter().filter(|&&v| v <= 100000).sum::<usize>())
            })
            .map(Box::new)?;

        Ok(result)
    }

    fn part2(&self, input: &str) -> Result<Box<dyn Display>> {
        let result = self
            .solve(input, |sizes| {
                let mut sizes = sizes.to_owned();
                sizes.sort();

                let free = 70000000 - sizes.iter().last().unwrap();
                sizes
                    .iter()
                    .find_map(|&v| (free + v >= 30000000).then_some(v))
                    .ok_or(anyhow::anyhow!("No result found"))
            })
            .map(Box::new)?;

        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use aoc::day_test;
    use paste::paste;

    day_test!(day07, Part1, "inputs/day07_demo.txt", "95437");
    day_test!(day07, Part2, "inputs/day07_demo.txt", "24933642");
}
