use std::{fmt::Display, ops::RangeInclusive};

use anyhow::Result;
use aoc::{Runnable, Solution};
use aoc_derive::Runner;
use itertools::Itertools;

#[derive(Runner)]
#[aoc(file = "inputs/day15.txt")]
pub struct DaySolution {
    pub filename: &'static str,
}

trait HasLen {
    fn len(&self) -> usize;
}

impl HasLen for RangeInclusive<isize> {
    fn len(&self) -> usize {
        (self.end() - self.start()) as usize
    }
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

    fn manhattan(&self, other: &Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

struct System {
    sensor: Point,
    distance: usize,
}

impl System {
    fn new(sensor: Point, beacon: Point) -> Self {
        let distance = sensor.manhattan(&beacon);
        Self { sensor, distance }
    }

    fn range(&self, y: isize) -> Option<RangeInclusive<isize>> {
        let offset = self.distance as isize - (self.sensor.y - y).abs();
        (offset >= 0).then_some(self.sensor.x - offset..=self.sensor.x + offset)
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

    fn combine_ranges(&self, ranges: Vec<RangeInclusive<isize>>) -> Vec<RangeInclusive<isize>> {
        let mut ranges = ranges;
        ranges.sort_by_key(|r| (*r.start()));

        let mut result = vec![];
        let mut current = ranges[0].clone();

        for range in ranges.iter().skip(1) {
            if range.start() <= current.end() {
                current = *current.start()..=*range.end().max(current.end());
            } else {
                result.push(current);
                current = range.clone();
            }
        }
        result.push(current);
        result
    }
}

impl Solution for DaySolution {
    fn part1(&self, input: &str) -> Result<Box<dyn Display>> {
        let items = self
            .parse(input)
            .iter()
            .map(|&(sensor, beacon)| System::new(sensor, beacon))
            .collect::<Vec<_>>();

        let line = if input.contains("Sensor at x=2, y=18") {
            10
        } else {
            2_000_000
        };

        let ranges = items
            .iter()
            .filter_map(|system| system.range(line))
            .collect::<Vec<_>>();

        let count: usize = self.combine_ranges(ranges).iter().map(|r| r.len()).sum();
        Ok(Box::new(count))
    }

    fn part2(&self, input: &str) -> Result<Box<dyn Display>> {
        let items = self
            .parse(input)
            .iter()
            .map(|&(sensor, beacon)| System::new(sensor, beacon))
            .collect::<Vec<_>>();

        let size = if input.contains("Sensor at x=2, y=18") {
            20
        } else {
            4_000_000
        };

        for y in 0..size {
            let ranges = items
                .iter()
                .filter_map(|system| system.range(y))
                .collect::<Vec<_>>();

            let ranges = self.combine_ranges(ranges);

            if ranges.len() <= 1 {
                continue;
            }

            if let Some(x) = ranges.iter().tuple_windows().find_map(|(a, b)| {
                if a.end() + 2 == *b.start() {
                    Some(a.end() + 1)
                } else {
                    None
                }
            }) {
                return Ok(Box::new(x * 4_000_000 + y));
            }
        }

        Err(anyhow::anyhow!("Not found"))
    }
}

#[cfg(test)]
mod test {
    use std::ops::RangeInclusive;

    use crate::*;
    use aoc::day_test;
    use paste::paste;

    day_test!(day15, Part1, "inputs/day15_demo.txt", "26");
    day_test!(day15, Part2, "inputs/day15_demo.txt", "56000011");

    #[test]
    fn test_day15_range() {
        use super::{HasLen, Point, System};
        let system = System::new(Point::new((0, 0)), Point::new((-3, -2)));
        assert_eq!(system.distance, 5);

        let range = system.range(-4);
        assert_eq!(range, Some(RangeInclusive::new(-1, 1)));
        assert_eq!(range.unwrap().len(), 2);
    }
}
