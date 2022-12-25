use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use anyhow::Result;
use aoc::{Runnable, Solution};
use aoc_derive::Runner;
use itertools::Itertools;
use pathfinding::prelude::astar;

#[derive(Runner)]
#[aoc(file = "inputs/day24.txt")]
pub struct DaySolution {
    pub filename: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Self {
        match c {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => unreachable!(),
        }
    }

    fn offset(&self) -> Position {
        match self {
            Direction::Up => Position { x: 0, y: -1 },
            Direction::Down => Position { x: 0, y: 1 },
            Direction::Left => Position { x: -1, y: 0 },
            Direction::Right => Position { x: 1, y: 0 },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl std::ops::Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Blizzard {
    position: Position,
    direction: Direction,
}

impl Blizzard {
    fn tick(&mut self, max_x: i32, max_y: i32) {
        self.position = self.position + self.direction.offset();

        if self.position.x <= 0 && self.direction == Direction::Left {
            self.position.x = max_x;
        } else if self.position.x > max_x && self.direction == Direction::Right {
            self.position.x = 1;
        }

        if self.position.y <= 0 && self.direction == Direction::Up {
            self.position.y = max_y;
        } else if self.position.y > max_y && self.direction == Direction::Down {
            self.position.y = 1;
        }
    }
}

impl DaySolution {
    fn parse(&self, input: &str) -> Result<(Vec<Blizzard>, (i32, i32))> {
        let max_y = input.lines().count() as i32 - 2;
        let max_x = input.lines().next().unwrap().chars().count() as i32 - 2;

        let blizzards = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().flat_map(move |(x, c)| {
                    if c == '.' || c == '#' {
                        None
                    } else {
                        Some(Blizzard {
                            position: Position {
                                x: x as i32,
                                y: y as i32,
                            },
                            direction: Direction::from_char(c),
                        })
                    }
                })
            })
            .collect::<Vec<_>>();

        Ok((blizzards, (max_x, max_y)))
    }

    fn find_path(
        &self,
        start: Position,
        end: Position,
        blizzards: &[Blizzard],
        max_x: i32,
        max_y: i32,
        iteration: i32,
    ) -> (i32, i32) {
        let directions = vec![
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ];

        let mut cache = HashMap::new();
        let mut blizzards = blizzards.to_owned();

        for i in 0.. {
            let hashset = blizzards.iter().map(|b| b.position).collect::<HashSet<_>>();

            if cache.values().contains(&hashset) {
                break;
            }

            cache.insert(i, hashset);
            blizzards.iter_mut().for_each(|b| b.tick(max_x, max_y));
        }
        let cache_size = cache.len() as i32;

        let result = astar(
            &(start, iteration),
            |&(p, time)| {
                let mut next = vec![];
                let blizzard = cache.get(&((time + 1) % cache_size)).unwrap();

                for d in directions.iter() {
                    let possible = p + d.offset();

                    if possible == end {
                        next.push(((possible, time + 1), 1));
                        continue;
                    }

                    if possible.x <= 0
                        || possible.x > max_x
                        || possible.y <= 0
                        || possible.y > max_y
                    {
                        continue;
                    }

                    if !blizzard.contains(&possible) {
                        next.push(((possible, time + 1), 1));
                    }
                }

                if !blizzard.contains(&p) {
                    next.push(((p, time + 1), 1));
                }

                next
            },
            |(p, _)| {
                // manhattan distance
                (p.x - end.x).abs() + (p.y - end.y).abs() / 3
            },
            |&(p, _)| p == end,
        );

        let (path, cost) = result.unwrap();

        (path.last().unwrap().1, cost)
    }
}

impl Solution for DaySolution {
    fn part1(&self, input: &str) -> Result<Box<dyn Display>> {
        let (blizzards, (max_x, max_y)) = self.parse(input)?;

        let start = Position { x: 1, y: 0 };
        let end = Position {
            x: max_x,
            y: max_y + 1,
        };

        let (_, cost) = self.find_path(start, end, &blizzards, max_x, max_y, 0);
        Ok(Box::new(cost))
    }

    fn part2(&self, input: &str) -> Result<Box<dyn Display>> {
        let (blizzards, (max_x, max_y)) = self.parse(input)?;

        let start = Position { x: 1, y: 0 };
        let end = Position {
            x: max_x,
            y: max_y + 1,
        };

        let (iteration, cost) = self.find_path(start, end, &blizzards, max_x, max_y, 0);
        let (iteration, cost2) = self.find_path(end, start, &blizzards, max_x, max_y, iteration);
        let (_, cost3) = self.find_path(start, end, &blizzards, max_x, max_y, iteration);

        // Ok(Box::new("AAA"))
        Ok(Box::new(cost + cost2 + cost3))
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use aoc::day_test;
    use paste::paste;

    day_test!(day24, Part1, "inputs/day24_demo.txt", "18");
    day_test!(day24, Part2, "inputs/day24_demo.txt", "54");
}
