use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use anyhow::Result;
use aoc::{Runnable, Solution};
use aoc_derive::Runner;

#[derive(Runner)]
#[aoc(file = "inputs/day15.txt")]
pub struct DaySolution {
    pub filename: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new((x, y): (isize, isize)) -> Self {
        Self { x, y }
    }

    fn manhattan(&self, other: Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

impl DaySolution {
    fn parse(&self, input: &str) -> Vec<(Point, Point)> {
        let pattern = crate::regex!(
            "^Sensor at x=(-?\\d+), y=(-?\\d+): closest beacon is at x=(-?\\d+), y=(-?\\d+)$"
        );

        input
            .lines()
            .map(|line| {
                let captures = pattern.captures(line).unwrap();

                let sensor = (
                    captures.get(1).unwrap().as_str().parse().unwrap(),
                    captures.get(2).unwrap().as_str().parse().unwrap(),
                );
                let beacon = (
                    captures.get(3).unwrap().as_str().parse().unwrap(),
                    captures.get(4).unwrap().as_str().parse().unwrap(),
                );

                (Point::new(sensor), Point::new(beacon))
            })
            .collect()
    }
}

impl Solution for DaySolution {
    fn part1(&self, input: &str) -> Result<Box<dyn Display>> {
        let items = self.parse(input);

        let mut map = HashMap::new();
        for (sensor, beacon) in items {
            map.insert(sensor, 'S');
            map.insert(beacon, 'B');
        }

        // let sensors = HashSet::<Point>::from_iter(items.iter().map(|&(sensor, _)| sensor));
        // let beacons = HashSet::<Point>::from_iter(items.iter().map(|&(_, beacon)| beacon));

        Ok(Box::new(0))
    }

    fn part2(&self, input: &str) -> Result<Box<dyn Display>> {
        Ok(Box::new(0))
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use aoc::day_test;
    use paste::paste;

    day_test!(day15, Part1, "inputs/day15_demo.txt", "unknown");
    day_test!(day15, Part2, "inputs/day15_demo.txt", "unknown");
}
