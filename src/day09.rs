use std::{collections::HashSet, fmt::Display, str::FromStr};

use anyhow::Result;
use aoc::{Runnable, Solution};
use aoc_derive::Runner;

#[derive(Runner)]
#[aoc(file = "inputs/day09.txt")]
pub struct DaySolution {
    pub filename: &'static str,
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Debug)]
struct Move {
    direction: Direction,
    steps: i32,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Position(i32, i32);

struct Rope(Vec<Position>);

impl Rope {
    fn new(size: usize) -> Self {
        Self(vec![Position(0, 0); size])
    }

    fn move_direction(&mut self, direction: Direction) {
        let mut position = self.0[0];

        match direction {
            Direction::Right => position.0 += 1,
            Direction::Left => position.0 -= 1,
            Direction::Up => position.1 -= 1,
            Direction::Down => position.1 += 1,
        }

        self.0[0] = position;
        for i in 1..self.0.len() {
            let other = self.0[i - 1];
            let mut current = self.0[i];

            if !current.touching(&other) {
                current.follow(&other);
                self.0[i] = current;
            }
        }
    }
}

impl Position {
    fn touching(&self, other: &Self) -> bool {
        let (dx, dy) = (self.0 - other.0, self.1 - other.1);
        dx.abs() <= 1 && dy.abs() <= 1
    }

    fn follow(&mut self, other: &Self) {
        let (dx, dy) = (self.0 - other.0, self.1 - other.1);

        self.0 -= dx.signum();
        self.1 -= dy.signum();
    }
}

impl FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (direction, value) = s
            .split_once(' ')
            .ok_or_else(|| anyhow::anyhow!("Invalid move: {}", s))?;
        let steps = value.parse::<i32>()?;

        let direction = match direction {
            "R" => Direction::Right,
            "L" => Direction::Left,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => anyhow::bail!("Invalid direction: {}", direction),
        };

        Ok(Self { direction, steps })
    }
}

impl Solution for DaySolution {
    /*
    Part 1: 6098
    Part 2: 2597
    */

    fn part1(&self, input: &str) -> Result<Box<dyn Display>> {
        let mut rope = Rope::new(2);
        let mut history = HashSet::new();

        for line in input.lines() {
            let r#move = line.parse::<Move>()?;
            for _ in 0..r#move.steps {
                rope.move_direction(r#move.direction);
                history.insert(*rope.0.last().unwrap());
            }
        }

        Ok(Box::new(history.len()))
    }

    fn part2(&self, input: &str) -> Result<Box<dyn Display>> {
        let mut rope = Rope::new(10);
        let mut history = HashSet::new();

        for line in input.lines() {
            let r#move = line.parse::<Move>()?;
            for _ in 0..r#move.steps {
                rope.move_direction(r#move.direction);
                history.insert(*rope.0.last().unwrap());
            }
        }

        Ok(Box::new(history.len()))
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use aoc::day_test;
    use paste::paste;

    day_test!(day09, Part1, "inputs/day09_demo.txt", "13");
    day_test!(day09, Part2, "inputs/day09_demo.txt", "1");

    // Part 1: 6098
    // Part 2: 2597
}
