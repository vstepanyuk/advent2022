use std::{collections::HashMap, fmt::Display};

use anyhow::Result;
use aoc::{Runnable, Solution};
use aoc_derive::Runner;

use num::complex::Complex;

#[derive(Runner)]
#[aoc(file = "inputs/day21.txt")]
pub struct DaySolution {
    pub filename: &'static str,
}

impl DaySolution {}

#[derive(parse_display::Display, parse_display::FromStr, Debug)]
enum Operation {
    #[display("{0} + {1}")]
    Add(String, String),
    #[display("{0} - {1}")]
    Subtract(String, String),
    #[display("{0} * {1}")]
    Multiply(String, String),
    #[display("{0} / {1}")]
    Divide(String, String),
    #[display("{0}")]
    Number(Complex<f32>),
}

#[derive(parse_display::Display, parse_display::FromStr, Debug)]
#[display("{name}: {operation}")]
struct Monkey {
    name: String,
    operation: Operation,
}

impl DaySolution {
    fn parse(&self, input: &str) -> HashMap<String, Operation> {
        input
            .lines()
            .map(|line| line.parse::<Monkey>().unwrap())
            .map(|m: Monkey| (m.name, m.operation))
            .collect::<HashMap<_, _>>()
    }

    fn compute(monkeys: &HashMap<String, Operation>, monkey: &str) -> Complex<f32> {
        match monkeys[monkey] {
            Operation::Number(n) => n,
            Operation::Add(ref a, ref b) => Self::compute(monkeys, a) + Self::compute(monkeys, b),
            Operation::Subtract(ref a, ref b) => {
                Self::compute(monkeys, a) - Self::compute(monkeys, b)
            }
            Operation::Multiply(ref a, ref b) => {
                Self::compute(monkeys, a) * Self::compute(monkeys, b)
            }
            Operation::Divide(ref a, ref b) => {
                Self::compute(monkeys, a) / Self::compute(monkeys, b)
            }
        }
    }
}

impl Solution for DaySolution {
    fn part1(&self, input: &str) -> Result<Box<dyn Display>> {
        let monkeys = self.parse(input);
        let result = Self::compute(&monkeys, "root").re;

        Ok(Box::new(result))
    }

    fn part2(&self, input: &str) -> Result<Box<dyn Display>> {
        let mut monkeys = self.parse(input);
        *monkeys.get_mut("humn").unwrap() = Operation::Number(Complex::new(0.0, 1.));

        let (left, right) = match &monkeys["root"] {
            Operation::Add(a, b)
            | Operation::Divide(a, b)
            | Operation::Multiply(a, b)
            | Operation::Subtract(a, b) => (a, b),
            _ => unreachable!(),
        };

        let (result1, result2) = (
            Self::compute(&monkeys, left),
            Self::compute(&monkeys, right),
        );

        let result = if result1.im == 0.0 {
            (result1.re - result2.re) / result2.im
        } else {
            (result2.re - result1.re) / result1.im
        };

        Ok(Box::new(result))
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use aoc::day_test;
    use paste::paste;

    day_test!(day21, Part1, "inputs/day21_demo.txt", "152");
    day_test!(day21, Part2, "inputs/day21_demo.txt", "301");
}
