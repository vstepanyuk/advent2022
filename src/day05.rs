use std::{collections::VecDeque, fmt::Display, str::FromStr};

use anyhow::Result;
use aoc::{Runnable, Solution};
use aoc_derive::Runner;

#[derive(Runner)]
#[aoc(file = "inputs/day05.txt")]
pub struct DaySolution {
    pub filename: &'static str,
}

#[derive(Debug, Clone, Copy)]
struct Action {
    count: usize,
    from: usize,
    to: usize,
}

impl FromStr for Action {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        if !s.starts_with("move ") {
            return Err(anyhow::anyhow!("Couldn't parse action"));
        }

        let mut parts = s.split_whitespace();

        let count = parts
            .nth(1)
            .ok_or_else(|| anyhow::anyhow!("Couldn't parse 'count'"))?
            .parse()?;

        let from = parts
            .nth(1)
            .ok_or_else(|| anyhow::anyhow!("Couldn't parse 'from'"))?
            .parse::<usize>()?
            - 1;

        let to = parts
            .nth(1)
            .ok_or_else(|| anyhow::anyhow!("Couldn't parse 'to'"))?
            .parse::<usize>()?
            - 1;

        Ok(Action { count, from, to })
    }
}

impl DaySolution {
    fn parse_stacks(&self, input: &str) -> Vec<VecDeque<char>> {
        let stacks_num = (input.lines().next().unwrap().len() + 1) / 4;

        input.lines().take_while(|line| line.contains('[')).fold(
            vec![VecDeque::new(); stacks_num],
            |mut stacks, line: &str| {
                for (i, stack) in stacks.iter_mut().enumerate().take(stacks_num) {
                    let item = line[i * 4 + 1..].chars().next().unwrap();
                    if !item.is_whitespace() {
                        stack.push_back(item);
                    }
                }

                stacks
            },
        )
    }

    fn parse_actions(&self, input: &str) -> Vec<Action> {
        input.lines().flat_map(|line| line.parse().ok()).collect()
    }

    fn rearrange<F>(&self, input: &str, mut get_item: F) -> String
    where
        F: FnMut(&mut VecDeque<char>) -> Option<char>,
    {
        let mut stacks = self.parse_stacks(input);
        let actions = self.parse_actions(input);

        actions.iter().for_each(|action| {
            let mut tmp = stacks[action.from]
                .drain(..action.count)
                .collect::<VecDeque<_>>();

            while let Some(item) = get_item(&mut tmp) {
                stacks[action.to].push_front(item);
            }
        });

        stacks
            .iter()
            .map_while(|stack| stack.front())
            .collect::<String>()
    }
}

impl Solution for DaySolution {
    fn part1(&self, input: &str) -> Result<Box<dyn Display>> {
        let result = self.rearrange(input, VecDeque::pop_front);

        Ok(Box::new(result))
    }

    fn part2(&self, input: &str) -> Result<Box<dyn Display>> {
        let result = self.rearrange(input, VecDeque::pop_back);

        Ok(Box::new(result))
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use aoc::day_test;
    use paste::paste;

    day_test!(day05, Part1, "inputs/day05_demo.txt", "CMZ");
    day_test!(day05, Part2, "inputs/day05_demo.txt", "MCD");
}
