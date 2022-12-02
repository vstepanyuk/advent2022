use std::{fmt::Display, str::FromStr};

use anyhow::Result;
use aoc::{Boxed, Runnable, Solution};
use aoc_derive::Runner;

#[derive(Runner)]
#[aoc(file = "inputs/day02.txt")]
pub struct DaySolution {
    pub filename: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Hand {
    Rock = 1,     // Lose
    Paper = 2,    // Draw
    Scissors = 3, // Win
}

impl std::ops::Add<i32> for Hand {
    type Output = i32;

    fn add(self, rhs: i32) -> Self::Output {
        self as i32 + rhs
    }
}

impl Hand {
    fn find_winner(&self) -> Self {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }

    fn find_loser(&self) -> Self {
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }

    fn play_reverse(&self, other: Hand) -> i32 {
        match self {
            Self::Rock => other.find_loser() + 0,
            Self::Paper => other + 3,
            Self::Scissors => other.find_winner() + 6,
        }
    }

    fn play(&self, other: Hand) -> i32 {
        *self
            + if *self == other.find_winner() {
                6
            } else if *self == other {
                3
            } else {
                0
            }
    }
}

impl FromStr for Hand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "A" | "X" => Ok(Hand::Rock),
            "B" | "Y" => Ok(Hand::Paper),
            "C" | "Z" => Ok(Hand::Scissors),
            _ => Err(anyhow::anyhow!("Invalid hand")),
        }
    }
}

impl DaySolution {
    fn parse(&self, input: &str) -> Result<Vec<(Hand, Hand)>> {
        input
            .lines()
            .map(|line| {
                let (first, second) = line.split_once(' ').unwrap();
                Ok((first.parse()?, second.parse()?))
            })
            .collect::<Result<Vec<_>>>()
    }
}

impl Solution for DaySolution {
    fn part1(&self, input: &str) -> Result<Box<dyn Display>> {
        let result = self
            .parse(input)?
            .iter()
            .map(|round| round.1.play(round.0))
            .sum::<i32>()
            .boxed();

        Ok(result)
    }

    fn part2(&self, input: &str) -> Result<Box<dyn Display>> {
        let result = self
            .parse(input)?
            .iter()
            .map(|round| round.1.play_reverse(round.0))
            .sum::<i32>()
            .boxed();

        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use aoc::day_test;
    use paste::paste;

    day_test!(day02, Part1, "inputs/day02_demo.txt", "15");
    day_test!(day02, Part2, "inputs/day02_demo.txt", "12");
}
