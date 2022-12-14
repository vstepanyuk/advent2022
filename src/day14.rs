use std::{collections::HashMap, fmt::Display, str::FromStr};

use anyhow::Result;
use aoc::{Runnable, Solution};
use aoc_derive::Runner;
use itertools::Itertools;

#[derive(Runner)]
#[aoc(file = "inputs/day14.txt")]
pub struct DaySolution {
    pub filename: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }

    fn fall_in<T>(&self, space: &HashMap<Self, T>) -> Option<Self> {
        [(0, 1), (-1, 1), (1, 1)]
            .iter()
            .map(|(dx, dy)| Point::new((self.x + dx, self.y + dy)))
            .find(|point| !space.contains_key(point))
    }

    fn between(start: &Point, end: &Point) -> impl Iterator<Item = Point> {
        itertools::iproduct!(
            start.x.min(end.x)..=start.x.max(end.x),
            start.y.min(end.y)..=start.y.max(end.y)
        )
        .map(Point::new)
    }
}

impl FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (x, y) = s
            .split_once(',')
            .ok_or_else(|| anyhow::anyhow!("Invalid point!"))?;

        Ok(Point {
            x: x.parse()?,
            y: y.parse()?,
        })
    }
}

impl DaySolution {
    fn parse(&self, input: &str) -> HashMap<Point, char> {
        HashMap::from_iter(input.lines().flat_map(|line| {
            line.split(" -> ")
                .flat_map(|s| s.parse::<Point>().ok())
                .tuple_windows()
                .flat_map(|(start, end)| Point::between(&start, &end).map(|p| (p, '#')))
        }))
    }
}

impl Solution for DaySolution {
    fn part1(&self, input: &str) -> Result<Box<dyn Display>> {
        let mut cave = self.parse(input);
        let max_height = cave.keys().map(|p| p.y).max().unwrap();
        let mut sand_point = Point::new((500, 0));

        loop {
            if let Some(new_sand_point) = sand_point.fall_in(&cave) {
                if new_sand_point.y > max_height {
                    break;
                }

                sand_point = new_sand_point;
                continue;
            }

            cave.insert(sand_point, 'o');
            sand_point = Point::new((500, 0));
        }

        Ok(Box::new(cave.values().filter(|&c| *c == 'o').count()))
    }

    fn part2(&self, input: &str) -> Result<Box<dyn Display>> {
        let mut cave = self.parse(input);
        let max_height = cave.keys().map(|p| p.y).max().unwrap();
        let mut sand_point = Point::new((500, 0));

        cave.extend(
            (0..1_000)
                .map(|x| (Point::new((x, max_height + 2)), '#'))
                .collect::<HashMap<_, _>>(),
        );

        loop {
            if let Some(new_sand_point) = sand_point.fall_in(&cave) {
                sand_point = new_sand_point;
                continue;
            }

            cave.insert(sand_point, 'o');
            if sand_point.x == 500 && sand_point.y == 0 {
                break;
            }

            sand_point = Point::new((500, 0));
        }

        Ok(Box::new(cave.values().filter(|&c| *c == 'o').count()))
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use aoc::day_test;
    use paste::paste;

    day_test!(day14, Part1, "inputs/day14_demo.txt", "24");
    day_test!(day14, Part2, "inputs/day14_demo.txt", "93");
}
