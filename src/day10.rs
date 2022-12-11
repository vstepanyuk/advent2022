use std::{fmt::Display, str::FromStr};

use anyhow::Result;
use aoc::{Runnable, Solution};
use aoc_derive::Runner;
use itertools::{process_results, Itertools};

#[derive(Runner)]
#[aoc(file = "inputs/day10.txt")]
pub struct DaySolution {
    pub filename: &'static str,
}

#[derive(Debug)]
enum Command {
    Noop,
    Addx(i32),
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "noop" => Self::Noop,
            _ => {
                let (_, value) = s
                    .split_once(' ')
                    .ok_or_else(|| anyhow::anyhow!("Invalid command"))?;

                let value = value.parse::<i32>()?;

                Self::Addx(value)
            }
        })
    }
}

impl DaySolution {
    fn parse(&self, input: &str) -> Result<Vec<i32>> {
        process_results(input.lines().map(Command::from_str), |iter| {
            iter.fold(vec![1], |mut x, command| {
                let prev = x[x.len() - 1];
                x.push(prev);

                if let Command::Addx(value) = command {
                    x.push(prev + value);
                }

                x
            })
        })
    }
}

impl Solution for DaySolution {
    fn part1(&self, input: &str) -> Result<Box<dyn Display>> {
        let x = self.parse(input)?;

        Ok(Box::new(
            x.iter()
                .enumerate()
                .skip(20)
                .step_by(40)
                .map(|(idx, _)| idx as i32 * x[idx - 1])
                .sum::<i32>(),
        ))
    }

    fn part2(&self, input: &str) -> Result<Box<dyn Display>> {
        let result = self
            .parse(input)?
            .iter()
            .enumerate()
            .fold(vec![vec!['.'; 40]; 6], |mut crt, (idx, pos)| {
                let (x, y) = (idx as i32 % 40, idx / 40);

                if (x - *pos).abs() <= 1 {
                    crt[y][x as usize] = '#';
                }

                crt
            })
            .iter()
            .map(|row| row.iter().collect::<String>())
            .join("\n");

        Ok(Box::new(format!("\n{}", result)))
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use aoc::day_test;
    use paste::paste;

    day_test!(day10, Part1, "inputs/day10_demo.txt", "13140");
    day_test!(
        day10,
        Part2,
        "inputs/day10_demo.txt",
        "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
    );
}
