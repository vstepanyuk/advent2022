use std::{cmp::Reverse, collections::BinaryHeap, fmt::Display, str::FromStr};

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
    fn recursive(
        entries: &[ShellLogEntry],
        start: usize,
        results: &mut BinaryHeap<Reverse<usize>>,
    ) -> (usize, usize) {
        let mut index = start;
        let mut total_size = 0;

        while index < entries.len() {
            match &entries[index] {
                ShellLogEntry::ChangeDirCommand(dir) => {
                    if dir == ".." {
                        break;
                    } else {
                        let (new_index, size) = Self::recursive(entries, index + 1, results);
                        index = new_index;
                        total_size += size;
                    }
                }
                ShellLogEntry::File(_, size) => {
                    total_size += size;
                }
                _ => {}
            }

            index += 1;
        }

        results.push(Reverse(total_size));
        (index, total_size)
    }

    fn solve<F>(&self, input: &str, mut solver: F) -> Result<usize>
    where
        F: FnMut(&mut BinaryHeap<Reverse<usize>>) -> Result<usize>,
    {
        let log = input
            .lines()
            .filter_map(|l| l.parse::<ShellLogEntry>().ok())
            .collect::<Vec<_>>();

        let mut results = BinaryHeap::new();
        _ = Self::recursive(&log, 0, &mut results);

        solver(&mut results)
    }
}

impl Solution for DaySolution {
    fn part1(&self, input: &str) -> Result<Box<dyn Display>> {
        let result = self
            .solve(input, |sizes| {
                Ok(sizes
                    .iter()
                    .map(|v| v.0)
                    .filter(|v| *v <= 100000)
                    .sum::<usize>())
            })
            .map(Box::new)?;

        Ok(result)
    }

    fn part2(&self, input: &str) -> Result<Box<dyn Display>> {
        let result = self
            .solve(input, |sizes| {
                let free = 70000000 - sizes.iter().last().unwrap().0;

                while let Some(Reverse(result)) = sizes.pop() {
                    if free + result >= 30000000 {
                        return Ok(result);
                    }
                }

                Err(anyhow::anyhow!("No result found"))
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
