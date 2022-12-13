use std::{cmp::Ordering, fmt::Display};

use anyhow::Result;
use aoc::{Runnable, Solution};
use aoc_derive::Runner;
use itertools::Itertools;

#[derive(Runner)]
#[aoc(file = "inputs/day13.txt")]
pub struct DaySolution {
    pub filename: &'static str,
}

type JsonValue = serde_json::Value;

impl DaySolution {
    fn parse(&self, input: &str) -> Result<Vec<JsonValue>> {
        let mut results = vec![];

        for line in input.lines() {
            if line.trim().is_empty() {
                continue;
            }

            results.push(line.parse::<JsonValue>()?);
        }

        Ok(results)
    }

    fn check(left: &JsonValue, right: &JsonValue) -> Ordering {
        // Two integers
        if let (Some(left), Some(right)) = (left.as_i64(), right.as_i64()) {
            return left.cmp(&right);
        }

        // Two arrays
        if let (Some(left), Some(right)) = (left.as_array(), right.as_array()) {
            for i in 0..left.len().max(right.len()) {
                if i == left.len() {
                    return Ordering::Less;
                }

                if i == right.len() {
                    return Ordering::Greater;
                }

                let result = Self::check(&left[i], &right[i]);
                if result == Ordering::Equal {
                    continue;
                }

                return result;
            }

            return Ordering::Equal;
        }

        if let Some(right) = right.as_i64() {
            return Self::check(left, &serde_json::to_value(vec![right]).unwrap());
        }

        Self::check(
            &serde_json::to_value(vec![left.as_i64().unwrap()]).unwrap(),
            right,
        )
    }
}

impl Solution for DaySolution {
    fn part1(&self, input: &str) -> Result<Box<dyn Display>> {
        let items = self.parse(input)?;
        let result = items
            .iter()
            .tuples::<(_, _)>()
            .map(|(left, right)| Self::check(left, right))
            .enumerate()
            .filter_map(|(i, r)| {
                if r == Ordering::Less {
                    return Some(i + 1);
                }

                None
            })
            .sum::<usize>();

        Ok(Box::new(result))
    }

    fn part2(&self, input: &str) -> Result<Box<dyn Display>> {
        let mut items = self.parse(input)?;
        items.push(serde_json::to_value(vec![vec![2]])?);
        items.push(serde_json::to_value(vec![vec![6]])?);

        items.sort_by(Self::check);

        let result = items
            .iter()
            .enumerate()
            .filter_map(|(i, v)| {
                let s = serde_json::to_string(v).unwrap();
                if s == "[[2]]" || s == "[[6]]" {
                    return Some(i + 1);
                }

                None
            })
            .product::<usize>();

        Ok(Box::new(result))
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use aoc::day_test;
    use paste::paste;

    day_test!(day13, Part1, "inputs/day13_demo.txt", "13");
    day_test!(day13, Part2, "inputs/day13_demo.txt", "140");
}
