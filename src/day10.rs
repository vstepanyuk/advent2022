use std::{fmt::Display, str::FromStr};

use anyhow::Result;
use aoc::{Runnable, Solution};
use aoc_derive::Runner;
use itertools::Itertools;

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

impl DaySolution {}

impl Solution for DaySolution {
    fn part1(&self, input: &str) -> Result<Box<dyn Display>> {
        let mut extended = vec![1];
        let mut index = 0;

        for line in input.lines() {
            let command = line.parse::<Command>().unwrap();
            match command {
                Command::Noop => {
                    extended.push(extended[index]);
                    index += 1;
                }
                Command::Addx(value) => {
                    extended.push(extended[index]);
                    extended.push(extended[index] + value);
                    index += 2;
                }
            }
        }

        let mut interval = 20;
        let mut result = 0;

        for idx in 0..extended.len() {
            if idx == interval {
                result += (idx) as i32 * extended[idx - 1];
                interval += 40;
            }
        }

        Ok(Box::new(result))
    }

    fn part2(&self, input: &str) -> Result<Box<dyn Display>> {
        let mut extended = vec![1];
        let mut index = 0;

        for line in input.lines() {
            let command = line.parse::<Command>().unwrap();
            match command {
                Command::Noop => {
                    extended.push(extended[index]);
                    index += 1;
                }
                Command::Addx(value) => {
                    extended.push(extended[index]);
                    extended.push(extended[index] + value);
                    index += 2;
                }
            }
        }

        let mut crt = vec![vec!['.'; 40]; 6];

        for (idx, pos) in extended.iter().enumerate() {
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
