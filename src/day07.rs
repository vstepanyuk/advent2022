use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    fmt::Display,
    str::FromStr,
};

use anyhow::Result;
use aoc::{Runnable, Solution};
use aoc_derive::Runner;
use itertools::Itertools;

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
        &self,
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
                        let (new_index, size) = self.recursive(entries, index + 1, results);
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
}

impl Solution for DaySolution {
    fn part1(&self, input: &str) -> Result<Box<dyn Display>> {
        let log = input
            .lines()
            .map(|l| l.parse::<ShellLogEntry>().unwrap())
            .collect::<Vec<_>>();

        let mut results = BinaryHeap::new();
        _ = self.recursive(&log, 0, &mut results);

        let result = results
            .iter()
            .map(|v| v.0)
            .filter(|v| *v <= 100000)
            .sum::<usize>();

        Ok(Box::new(result))
    }

    fn part2(&self, input: &str) -> Result<Box<dyn Display>> {
        let log = input
            .lines()
            .map(|l| l.parse::<ShellLogEntry>().unwrap())
            .collect::<Vec<_>>();

        let mut results = BinaryHeap::new();
        _ = self.recursive(&log, 0, &mut results);

        let free = 70000000 - results.iter().last().unwrap().0;

        while let Some(Reverse(result)) = results.pop() {
            if free + result >= 30000000 {
                return Ok(Box::new(result));
            }
        }

        Ok(Box::new(""))
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
