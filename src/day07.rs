use std::{collections::HashMap, fmt::Display, str::FromStr};

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
    fn parse(&self, input: &str) -> Result<HashMap<Vec<String>, usize>> {
        let mut result = HashMap::new();
        let mut path = vec![];

        for line in input.lines() {
            let item = line.parse::<ShellLogEntry>()?;

            match item {
                ShellLogEntry::ListCommand => {}
                ShellLogEntry::ChangeDirCommand(dir) => {
                    if dir == ".." {
                        _ = path.pop();
                    } else {
                        path.push(dir);
                    }
                }
                ShellLogEntry::Directory(_dir) => {}
                ShellLogEntry::File(_file, size) => {
                    self.update_size(&mut result, path.clone(), size);
                }
            }
        }

        Ok(result)
    }

    fn update_size(
        &self,
        result: &mut HashMap<Vec<String>, usize>,
        path: Vec<String>,
        size: usize,
    ) {
        *result.entry(path.clone()).or_insert(0) += size;

        for i in 1..path.len() {
            *result.entry(path[0..i].to_owned()).or_insert(0) += size;
        }
    }
}

impl Solution for DaySolution {
    fn part1(&self, input: &str) -> Result<Box<dyn Display>> {
        let result = self.parse(input)?;

        Ok(Box::new(
            result
                .values()
                .filter(|&size| *size <= 100000)
                .sum::<usize>(),
        ))
    }

    fn part2(&self, input: &str) -> Result<Box<dyn Display>> {
        let result = self.parse(input)?;
        let free = 70000000 - result.get(&vec!["/".to_string()]).unwrap();

        let a = *result
            .values()
            .sorted()
            .find(|&&v| free + v >= 30000000)
            .unwrap();

        Ok(Box::new(a))
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
