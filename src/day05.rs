use std::{collections::VecDeque, fmt::Display};

use anyhow::Result;
use aoc::{Runnable, Solution};
use aoc_derive::Runner;

#[derive(Runner)]
#[aoc(file = "inputs/day05.txt")]
pub struct DaySolution {
    pub filename: &'static str,
}

impl DaySolution {
    fn parse_stacks(&self, input: &str) -> Vec<VecDeque<char>> {
        let stacks_num = (input.lines().next().unwrap().len() + 1) / 4;

        input.lines().take_while(|line| line.contains('[')).fold(
            vec![VecDeque::new(); stacks_num],
            |mut stacks, line: &str| {
                for (i, stack) in stacks.iter_mut().enumerate().take(stacks_num) {
                    let start = 1 + i * 4;
                    let item = line[start..].chars().next().unwrap();
                    if !item.is_whitespace() {
                        stack.push_back(item);
                    }
                }

                stacks
            },
        )
    }

    fn parse_actions(&self, input: &str) -> Vec<(usize, usize, usize)> {
        input
            .lines()
            .skip_while(|line| !line.starts_with("move"))
            .map(|line| {
                let mut parts = line.split(' ');

                let size = parts.nth(1).unwrap().parse().unwrap();
                let from = parts.nth(1).unwrap().parse::<usize>().unwrap();
                let to = parts.nth(1).unwrap().parse::<usize>().unwrap();
                (size, from - 1, to - 1)
            })
            .collect()
    }

    fn rearrange<F>(&self, input: &str, mut action: F) -> String
    where
        F: FnMut(&mut Vec<VecDeque<char>>, usize, usize, usize),
    {
        let mut stacks = self.parse_stacks(input);
        let actions = self.parse_actions(input);

        for (size, from, to) in actions {
            action(&mut stacks, from, to, size);
        }

        stacks
            .iter()
            .map(|stack| stack.front().unwrap())
            .collect::<String>()
    }
}

impl Solution for DaySolution {
    fn part1(&self, input: &str) -> Result<Box<dyn Display>> {
        let result = self.rearrange(input, |stacks, from, to, size| {
            for _ in 0..size {
                let item = stacks[from].pop_front().unwrap();
                stacks[to].push_front(item);
            }
        });

        Ok(Box::new(result))
    }

    fn part2(&self, input: &str) -> Result<Box<dyn Display>> {
        let result = self.rearrange(input, |stacks, from, to, size| {
            let mut tmp = VecDeque::new();
            for _ in 0..size {
                let item = stacks[from].pop_front().unwrap();
                tmp.push_back(item);
            }

            while let Some(item) = tmp.pop_back() {
                stacks[to].push_front(item);
            }
        });

        Ok(Box::new(result))
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use aoc::day_test;
    use paste::paste;

    day_test!(day05, Part1, "inputs/day05_demo.txt", "CMZ");
    day_test!(day05, Part2, "inputs/day05_demo.txt", "MCD");
}
