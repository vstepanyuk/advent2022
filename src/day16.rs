use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use anyhow::Result;
use aoc::{Runnable, Solution};
use aoc_derive::Runner;
use itertools::Itertools;
use nom::{
    self, branch::alt, bytes::complete::tag, bytes::complete::take, character::complete::digit1,
    combinator::map_res, multi::separated_list1, sequence::tuple, IResult,
};

#[derive(Runner)]
#[aoc(file = "inputs/day16.txt")]
pub struct DaySolution {
    pub filename: &'static str,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Valve {
    id: String,
    rate: i32,
    connections: Vec<String>,
}

impl Valve {
    fn parse(i: &str) -> IResult<&str, Valve> {
        let (_, (_, id, _, rate, _, connections)) = tuple((
            tag("Valve "),
            take(2usize),
            tag(" has flow rate="),
            map_res(digit1, str::parse),
            alt((
                tag("; tunnels lead to valves "),
                tag("; tunnel leads to valve "),
            )),
            separated_list1(tag(", "), take(2usize)),
        ))(i)?;

        Ok((
            i,
            Valve {
                id: id.to_owned(),
                rate,
                connections: connections.into_iter().map(String::from).collect(),
            },
        ))
    }
}

impl DaySolution {
    fn parse(&self, input: &str) -> HashMap<String, Valve> {
        HashMap::from_iter(input.lines().map(|line| {
            let (_, valve) = Valve::parse(line).unwrap();
            (valve.id.clone(), valve)
        }))
    }
}

impl DaySolution {}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct State {
    opened: Vec<String>,
    you: String,
    total: i32,
    elephant: Option<String>,
}

impl State {
    fn new(current: impl ToString) -> Self {
        State {
            opened: vec![],
            you: current.to_string(),
            total: 0,
            elephant: None,
        }
    }

    fn release(&mut self, valves: &HashMap<String, Valve>) {
        for opened in self.opened.iter() {
            let valve = valves.get(opened).unwrap();
            self.total += valve.rate;
        }
    }
}

impl Solution for DaySolution {
    fn part1(&self, input: &str) -> Result<Box<dyn Display>> {
        let valves = self.parse(input);

        let openable = valves
            .iter()
            .filter(|(_, valve)| valve.rate > 0)
            .map(|(id, _)| id)
            .cloned()
            .collect::<Vec<_>>();

        let mut states = HashSet::new();
        let state = State::new("AA");
        states.insert(state);

        for _ in 0..30 {
            let mut new_states = HashSet::new();

            for state in states.iter() {
                let valve = valves.get(&state.you).unwrap();

                if openable.len() == state.opened.len() {
                    let mut new_state = state.clone();
                    new_state.release(&valves);
                    // All opened
                    new_states.insert(new_state);

                    continue;
                }

                if !state.opened.contains(&state.you) && valve.rate > 0 {
                    let mut new_state = state.clone();
                    new_state.release(&valves);

                    // Current is not opened
                    new_state.opened.push(state.you.clone());
                    new_states.insert(new_state);
                }

                // Current is opened
                for neighbour in valve.connections.iter() {
                    let mut neighbour_state = state.clone();
                    neighbour_state.release(&valves);

                    neighbour_state.you = neighbour.clone();
                    new_states.insert(neighbour_state);
                }
            }

            states = HashSet::from_iter(
                new_states
                    .into_iter()
                    .sorted_by_key(|s| s.total)
                    .rev()
                    .take(100),
            );
        }

        let result = states.iter().map(|s| s.total).max().unwrap();

        Ok(Box::new(result))
    }

    fn part2(&self, input: &str) -> Result<Box<dyn Display>> {
        let valves = self.parse(input);

        let openable = valves
            .iter()
            .filter(|(_, valve)| valve.rate > 0)
            .map(|(id, _)| id)
            .cloned()
            .collect::<Vec<_>>();

        let mut states = HashSet::new();
        let state = State::new("AA");
        states.insert(state);

        for _ in 0..26 {
            let mut new_states = HashSet::new();

            for state in states.iter() {
                // All opened
                if openable.len() == state.opened.len() {
                    let mut new_state = state.clone();
                    new_state.release(&valves);
                    new_states.insert(new_state);

                    continue;
                }

                let your_valve = valves.get(&state.you).unwrap();

                if !state.opened.contains(&state.you) && your_valve.rate > 0 {
                    let mut new_state = state.clone();
                    new_state.release(&valves);

                    // Current is not opened
                    new_state.opened.push(state.you.clone());
                    new_states.insert(new_state);
                }

                // Current is opened
                for neighbour in your_valve.connections.iter() {
                    let mut neighbour_state = state.clone();
                    neighbour_state.release(&valves);

                    neighbour_state.you = neighbour.clone();
                    new_states.insert(neighbour_state);
                }
            }

            states = HashSet::from_iter(
                new_states
                    .into_iter()
                    .sorted_by_key(|s| s.total)
                    .rev()
                    .take(100),
            );
        }

        let result = states.iter().map(|s| s.total).max().unwrap();

        Ok(Box::new(result))
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use aoc::day_test;
    use paste::paste;

    day_test!(day16, Part1, "inputs/day16_demo.txt", "1651");
    day_test!(day16, Part2, "inputs/day16_demo.txt", "1707");
}
