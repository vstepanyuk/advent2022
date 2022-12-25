use std::{fmt::Display, str::FromStr};

use anyhow::Result;
use aoc::{Runnable, Solution};
use aoc_derive::Runner;

#[derive(Runner)]
#[aoc(file = "inputs/day25.txt")]
pub struct DaySolution {
    pub filename: &'static str,
}

#[derive(Debug)]
struct Snafu(i128);

impl FromStr for Snafu {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let number = s.chars().fold(0, |number, c| {
            number * 5
                + match c {
                    '-' => -1,
                    '=' => -2,
                    x => x.to_digit(3).unwrap() as i128,
                }
        });

        Ok(Snafu(number))
    }
}

impl ToString for Snafu {
    fn to_string(&self) -> String {
        let mut number = self.0;
        let mut result = String::new();

        while number > 0 {
            let digit = number % 5;

            result.push(match digit {
                3 => '=',
                4 => '-',
                x => (x as u8 + b'0') as char,
            });

            if digit > 2 {
                number += 2;
            }
            number /= 5;
        }

        result.chars().rev().collect::<String>()
    }
}

impl Solution for DaySolution {
    fn part1(&self, input: &str) -> Result<Box<dyn Display>> {
        let sum = input
            .lines()
            .map(|line| line.parse::<Snafu>().unwrap())
            .map(|snafu| snafu.0)
            .sum::<i128>();

        Ok(Box::new(Snafu(sum).to_string()))
    }

    fn part2(&self, _input: &str) -> Result<Box<dyn Display>> {
        println!("{:?}", Snafu(4).to_string());

        Ok(Box::new("doesn't exist"))
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use aoc::day_test;
    use paste::paste;

    day_test!(day25, Part1, "inputs/day25_demo.txt", "2=-1=0");
}
