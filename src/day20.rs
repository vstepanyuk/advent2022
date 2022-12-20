use std::{collections::VecDeque, fmt::Display};

use anyhow::Result;
use aoc::{Runnable, Solution};
use aoc_derive::Runner;

#[derive(Runner)]
#[aoc(file = "inputs/day20.txt")]
pub struct DaySolution {
    pub filename: &'static str,
}

type ElementType = i128;

impl DaySolution {
    fn solve(&self, input: &str, key: ElementType, times: usize) -> ElementType {
        let mut arr = input
            .lines()
            .flat_map(|l| l.parse::<ElementType>().ok())
            .map(|value| value * key)
            .enumerate()
            .collect::<VecDeque<_>>();

        for _ in 0..times {
            for i in 0..arr.len() {
                let idx = arr
                    .iter()
                    .enumerate()
                    .position(|(_, &(idx, _))| idx == i)
                    .unwrap();

                arr.rotate_left(idx);
                let element = arr.pop_front().unwrap();
                let count = element.1.rem_euclid(arr.len() as ElementType) as usize;
                arr.rotate_left(count);
                arr.push_front(element);
            }
        }

        let zero_index = arr.iter().position(|(_, value)| value.eq(&0)).unwrap();

        [1000, 2000, 3000]
            .iter()
            .map(|i| arr[(zero_index + i).rem_euclid(arr.len())].1)
            .sum()
    }
}

impl Solution for DaySolution {
    fn part1(&self, input: &str) -> Result<Box<dyn Display>> {
        Ok(Box::new(self.solve(input, 1, 1)))
    }

    fn part2(&self, input: &str) -> Result<Box<dyn Display>> {
        Ok(Box::new(self.solve(input, 811589153, 10)))
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use aoc::day_test;
    use paste::paste;

    day_test!(day20, Part1, "inputs/day20_demo.txt", "3");
    day_test!(day20, Part2, "inputs/day20_demo.txt", "1623178306");
}
