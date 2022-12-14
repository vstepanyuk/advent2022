use std::{collections::HashMap, fmt::Display, str::FromStr};

use anyhow::Result;
use aoc::{Runnable, Solution};
use aoc_derive::Runner;
use itertools::process_results;

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
}

impl FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (x, y) = s
            .split_once(',')
            .ok_or_else(|| anyhow::anyhow!("Invalid point"))?;

        Ok(Point {
            x: x.parse()?,
            y: y.parse()?,
        })
    }
}

impl DaySolution {
    fn generate(&self, start: &Point, end: &Point) -> impl Iterator<Item = Point> {
        let range_x = if start.x < end.x {
            start.x..=end.x
        } else {
            end.x..=start.x
        };

        let range_y = if start.y < end.y {
            start.y..=end.y
        } else {
            end.y..=start.y
        };

        itertools::iproduct!(range_x, range_y).map(Point::new)
    }

    fn parse(&self, input: &str) -> Result<HashMap<Point, char>> {
        let mut cave = HashMap::new();

        for line in input.lines() {
            let points = process_results(line.split(" -> ").map(|s| s.parse::<Point>()), |iter| {
                iter.collect::<Vec<_>>()
            })?;

            points.windows(2).for_each(|items| {
                for point in self.generate(&items[0], &items[1]) {
                    cave.insert(point, '#');
                }
            });
        }

        Ok(cave)
    }

    #[allow(dead_code)]
    fn debug(&self, cave: &HashMap<Point, char>) {
        let min_x = cave.keys().map(|p| p.x).min().unwrap();
        let max_x = cave.keys().map(|p| p.x).max().unwrap();
        let max_y = cave.keys().map(|p| p.y).max().unwrap();

        for y in 0..=max_y {
            for x in min_x..=max_x {
                print!("{}", cave.get(&Point::new((x, y))).unwrap_or(&'.'));
            }

            println!();
        }
    }

    fn can_move(&self, cave: &HashMap<Point, char>, point: &Point) -> bool {
        !cave.contains_key(point)
    }
}

impl Solution for DaySolution {
    fn part1(&self, input: &str) -> Result<Box<dyn Display>> {
        let mut cave = self.parse(input)?;
        let max_height = cave.keys().map(|p| p.y).max().unwrap();
        let mut sand = Point::new((500, 0));

        loop {
            if sand.y > max_height {
                break;
            }

            if self.can_move(&cave, &Point::new((sand.x, sand.y + 1))) {
                sand.y += 1;
            } else if self.can_move(&cave, &Point::new((sand.x - 1, sand.y + 1))) {
                sand.y += 1;
                sand.x -= 1
            } else if self.can_move(&cave, &Point::new((sand.x + 1, sand.y + 1))) {
                sand.y += 1;
                sand.x += 1
            } else {
                cave.insert(sand, 'o');
                sand = Point::new((500, 0));
            }
        }

        Ok(Box::new(cave.values().filter(|&c| *c == 'o').count()))
    }

    fn part2(&self, input: &str) -> Result<Box<dyn Display>> {
        let mut cave = self.parse(input)?;
        let max_height = cave.keys().map(|p| p.y).max().unwrap();
        let mut sand = Point::new((500, 0));

        for x in 0..1_000 {
            cave.insert(Point::new((x, max_height + 2)), '#');
        }

        loop {
            if self.can_move(&cave, &Point::new((sand.x, sand.y + 1))) {
                sand.y += 1;
            } else if self.can_move(&cave, &Point::new((sand.x - 1, sand.y + 1))) {
                sand.y += 1;
                sand.x -= 1
            } else if self.can_move(&cave, &Point::new((sand.x + 1, sand.y + 1))) {
                sand.y += 1;
                sand.x += 1
            } else {
                cave.insert(sand, 'o');
                if sand.x == 500 && sand.y == 0 {
                    break;
                }

                sand = Point::new((500, 0));
            }
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
