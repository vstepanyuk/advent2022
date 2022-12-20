use std::{fmt::Display, str::FromStr};

use anyhow::Result;
use aoc::{Runnable, Solution};
use aoc_derive::Runner;
use pathfinding::prelude::bfs_reach;

#[derive(Runner)]
#[aoc(file = "inputs/day19.txt")]
pub struct DaySolution {
    pub filename: &'static str,
}

#[derive(Debug)]
struct Blueprint {
    ore_robot_cost: usize,
    clay_robot_cost: usize,
    obsidian_robot_cost: (usize, usize),
    geode_robot_cost: (usize, usize),
}

impl FromStr for Blueprint {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let items = s
            .split_ascii_whitespace()
            .flat_map(|a| a.parse::<usize>().ok())
            .collect::<Vec<_>>();

        Ok(Self {
            ore_robot_cost: items[0],
            clay_robot_cost: items[1],
            obsidian_robot_cost: (items[2], items[3]),
            geode_robot_cost: (items[4], items[5]),
        })
    }
}

#[derive(Debug, Default, Eq, PartialEq, Hash, Clone, Copy)]
struct State {
    ore_robots: usize,
    clay_robots: usize,
    obsidian_robots: usize,
    geode_robots: usize,
    geocodes: usize,
    ore: usize,
    clay: usize,
    obsidian: usize,
    time: usize,
}

impl State {
    fn new() -> Self {
        let mut state = Self::default();
        state.ore_robots = 1;
        state
    }

    fn increment(&mut self) {
        self.ore += self.ore_robots;
        self.clay += self.clay_robots;
        self.obsidian += self.obsidian_robots;
        self.geocodes += self.geode_robots;
        self.time -= 1;
    }
}

impl DaySolution {
    fn solve(&self, blueprint: &Blueprint, time: usize) -> usize {
        let mut state = State::new();
        state.time = time;

        let max_ore_cost = blueprint
            .ore_robot_cost
            .max(blueprint.clay_robot_cost)
            .max(blueprint.obsidian_robot_cost.0)
            .max(blueprint.geode_robot_cost.0);

        let mut result = 0;

        _ = bfs_reach(state, |state| {
            if state.time == 0 {
                result = result.max(state.geocodes);
                return vec![];
            }

            let mut new_states = vec![];

            if state.ore >= blueprint.geode_robot_cost.0
                && state.obsidian >= blueprint.geode_robot_cost.1
            {
                let mut new_state = state.clone();
                new_state.increment();
                new_state.geode_robots += 1;

                new_state.ore -= blueprint.geode_robot_cost.0;
                new_state.obsidian -= blueprint.geode_robot_cost.1;
                new_states.push(new_state);

                return new_states;
            }

            if state.ore >= blueprint.obsidian_robot_cost.0
                && state.clay >= blueprint.obsidian_robot_cost.1
            // && state.obsidian_robots < blueprint.geode_robot_cost.1
            {
                let mut new_state = state.clone();
                new_state.increment();
                new_state.obsidian_robots += 1;

                new_state.ore -= blueprint.obsidian_robot_cost.0;
                new_state.clay -= blueprint.obsidian_robot_cost.1;
                new_states.push(new_state);

                // return new_states;
            }

            if state.ore >= blueprint.ore_robot_cost && state.ore_robots < max_ore_cost {
                let mut new_state = state.clone();
                new_state.increment();
                new_state.ore_robots += 1;
                new_state.ore -= blueprint.ore_robot_cost;
                new_states.push(new_state);
            }

            if state.ore >= blueprint.clay_robot_cost {
                let mut new_state = state.clone();
                new_state.increment();
                new_state.clay_robots += 1;
                new_state.ore -= blueprint.clay_robot_cost;
                new_states.push(new_state);
            }

            let mut new_state = state.clone();
            new_state.increment();
            new_states.push(new_state);

            new_states
        })
        .count();

        result
    }
}

impl Solution for DaySolution {
    fn part1(&self, input: &str) -> Result<Box<dyn Display>> {
        let result: usize = input
            .lines()
            .map(|line| line.parse::<Blueprint>().unwrap())
            .enumerate()
            .map(|(id, blueprint)| self.solve(&blueprint, 24) * (id + 1))
            .sum();

        Ok(Box::new(result))
    }

    fn part2(&self, input: &str) -> Result<Box<dyn Display>> {
        let result: usize = input
            .lines()
            .map(|line| line.parse::<Blueprint>().unwrap())
            .map(|blueprint| self.solve(&blueprint, 32))
            .take(3)
            .product();

        Ok(Box::new(result))
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use aoc::day_test;
    use paste::paste;

    day_test!(day19, Part1, "inputs/day19_demo.txt", "33");
    day_test!(day19, Part2, "inputs/day19_demo.txt", "3472");
}
