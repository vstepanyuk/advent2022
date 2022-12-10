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

        let mut interval = 20;
        let mut result = 0;

        for idx in 0..x.len() {
            if idx == interval {
                result += (idx) as i32 * x[idx - 1];
                interval += 40;
            }
        }

        Ok(Box::new(result))
    }

    fn part2(&self, input: &str) -> Result<Box<dyn Display>> {
        let x = self.parse(input)?;
        let mut crt = vec![vec!['.'; 40]; 6];

        for (idx, pos) in x.iter().enumerate() {
            let x = (idx % 40) as i32;
            let y = idx / 40;

            if x - 1 == *pos || x + 1 == *pos || x == *pos {
                crt[y][x as usize] = '#';
            }
        }

        let result = crt
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
