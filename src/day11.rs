use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
    str::FromStr,
};

use anyhow::Result;
use aoc::{Runnable, Solution};
use aoc_derive::Runner;
use itertools::{process_results, Itertools};

#[derive(Runner)]
#[aoc(file = "inputs/day11.txt")]
pub struct DaySolution {
    pub filename: &'static str,
}

#[derive(Debug)]
enum Operand {
    Value(usize),
    Own,
}

#[derive(Debug)]
enum Operation {
    Add(Operand),
    Multiply(Operand),
}

impl Operation {
    fn perform(&self, other: usize) -> usize {
        match self {
            Self::Add(Operand::Value(value)) => value + other,
            Self::Add(Operand::Own) => other + other,
            Self::Multiply(Operand::Value(value)) => other * value,
            Self::Multiply(Operand::Own) => other * other,
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<usize>,
    op: Operation,
    test: usize,
    branch: (usize, usize), // false, true
}

impl Monkey {
    fn inspect(&mut self, base: usize, divider: Option<usize>) -> (Vec<usize>, Vec<usize>) {
        let mut left = vec![];
        let mut right = vec![];

        while let Some(item) = self.items.pop_front() {
            let value = self.op.perform(item) / divider.unwrap_or(1);

            if value % self.test == 0 {
                right.push(value % base);
            } else {
                left.push(value % base);
            }
        }

        (left, right)
    }
}

impl FromStr for Monkey {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut items = VecDeque::new();
        let mut op = None;
        let mut test = 0;
        let mut left_branch = 0;
        let mut right_branch = 0;

        for line in s.lines() {
            let (prefix, payload) = line
                .split_once(':')
                .ok_or_else(|| anyhow::anyhow!("Invalid line"))?;

            let payload = payload.trim();

            match prefix.trim() {
                "Starting items" => {
                    items =
                        process_results(payload.split(", ").map(|v| v.parse()), |i| i.collect())?;
                }
                "Operation" => {
                    let payload = payload.replace("new = old ", "");
                    let (operation, right) = payload
                        .split_once(' ')
                        .ok_or_else(|| anyhow::anyhow!("Invalid operation"))?;

                    let right = match right {
                        "old" => Operand::Own,
                        _ => Operand::Value(right.parse::<usize>()?),
                    };

                    op = Some(match operation {
                        "+" => Operation::Add(right),
                        "*" => Operation::Multiply(right),
                        _ => return Err(anyhow::anyhow!("Invalid operation")),
                    });
                }
                "Test" => {
                    test = payload.replace("divisible by ", "").parse()?;
                }
                "If true" => {
                    right_branch = payload.replace("throw to monkey ", "").parse::<usize>()?;
                }
                "If false" => {
                    left_branch = payload.replace("throw to monkey ", "").parse::<usize>()?;
                }
                _ => continue,
            }
        }

        Ok(Self {
            items,
            op: op.unwrap(),
            test,
            branch: (left_branch, right_branch),
        })
    }
}

impl DaySolution {
    fn solve(&self, input: &str, rounds: usize, divider: Option<usize>) -> Result<usize> {
        let mut monkeys: Vec<Monkey> =
            process_results(input.split("\n\n").map(|b| b.parse()), |i| i.collect())?;

        let mut counts = HashMap::new();
        let base = monkeys.iter().map(|m| m.test).product();

        for _ in 0..rounds {
            for index in 0..monkeys.len() {
                *counts.entry(index).or_insert(0) += monkeys[index].items.len();

                let branch = monkeys[index].branch;
                let (left, right) = monkeys[index].inspect(base, divider);
                monkeys[branch.0].items.extend(left);
                monkeys[branch.1].items.extend(right);
            }
        }

        Ok(counts.values().sorted().rev().take(2).product())
    }
}

impl Solution for DaySolution {
    fn part1(&self, input: &str) -> Result<Box<dyn Display>> {
        Ok(Box::new(self.solve(input, 20, Some(3))?))
    }

    fn part2(&self, input: &str) -> Result<Box<dyn Display>> {
        Ok(Box::new(self.solve(input, 10_000, None)?))
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use aoc::day_test;
    use paste::paste;

    day_test!(day11, Part1, "inputs/day11_demo.txt", "10605");
    day_test!(day11, Part2, "inputs/day11_demo.txt", "2713310158");
}
