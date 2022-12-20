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

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct State {
    opened: Vec<String>,
    me: String,
    total: i32,
    elephant: Option<String>,
}

impl State {
    fn new(current: impl ToString) -> Self {
        State {
            opened: vec![],
            me: current.to_string(),
            total: 0,
            elephant: None,
        }
    }

    fn release(&mut self, valves: &HashMap<String, Valve>) {
        let mut released = 0;
        for opened in self.opened.iter() {
            let valve = valves.get(opened).unwrap();
            released += valve.rate;
        }

        self.total += released;
    }

    fn my_possible_actions(
        &self,
        valves: &HashMap<String, Valve>,
        possible_open: usize,
    ) -> Vec<State> {
        let mut actions = vec![];
        let valve = valves.get(&self.me).unwrap();

        // Open current
        if !self.opened.contains(&self.me) && valve.rate > 0 {
            let mut new_state = self.clone();
            new_state.opened.push(self.me.clone());
            actions.push(new_state);
        }

        // Only release if all opened
        if possible_open == self.opened.len() {
            actions.push(self.clone());
        }

        // Move to neighbour
        for neighbour in valve.connections.iter() {
            let mut neighbour_state = self.clone();
            neighbour_state.me = neighbour.clone();

            actions.push(neighbour_state);
        }

        actions
    }

    fn is_opening(&self, other: &State) -> bool {
        self.me == other.me
    }

    fn my_possible_actions_with_elephant(
        &self,
        valves: &HashMap<String, Valve>,
        possible_open: usize,
    ) -> Vec<State> {
        if self.opened.len() == possible_open {
            return vec![self.clone()];
        }

        let my_actions = self.my_possible_actions(valves, possible_open);
        let mut actions = vec![];

        for my_action in my_actions.iter() {
            if self.elephant.is_none() {
                // moving elephant
                actions.extend(
                    valves[&self.me]
                        .connections
                        .iter()
                        .cloned()
                        .filter(|x| {
                            x != if self.is_opening(my_action) {
                                &self.me
                            } else {
                                &my_action.me
                            }
                        })
                        .map(|neighbour| {
                            let mut neighbour_state = my_action.clone();
                            neighbour_state.elephant = Some(neighbour);
                            neighbour_state
                        }),
                );
            } else {
                let elephant = self.elephant.clone().unwrap();
                let elephant_valve = valves.get(&elephant).unwrap();

                // open
                if elephant_valve.rate > 0
                    && !self.opened.contains(&elephant)
                    && (!self.is_opening(my_action) || self.me != elephant)
                {
                    let mut new_state = my_action.clone();
                    new_state.opened.push(elephant.clone());

                    actions.push(new_state);
                }

                // moving
                actions.extend(
                    valves[&elephant]
                        .connections
                        .iter()
                        .cloned()
                        .filter(|x| x != &self.me)
                        .map(|neighbour| {
                            let mut neighbour_state = my_action.clone();
                            neighbour_state.elephant = Some(neighbour);
                            neighbour_state
                        }),
                );
            }
        }

        actions
    }
}

impl DaySolution {
    fn parse(&self, input: &str) -> HashMap<String, Valve> {
        HashMap::from_iter(input.lines().map(|line| {
            let (_, valve) = Valve::parse(line).unwrap();
            (valve.id.clone(), valve)
        }))
    }

    fn solve<F>(&self, input: &str, minutes: usize, states_limit: usize, next_states: F) -> i32
    where
        F: Fn(&State, &HashMap<String, Valve>, usize) -> Vec<State>,
    {
        let valves = self.parse(input);

        let openable = valves.iter().filter(|(_, valve)| valve.rate > 0).count();

        let mut states = HashSet::new();
        let state = State::new("AA");
        states.insert(state);

        for _ in 0..minutes {
            let mut new_states = HashSet::new();

            for state in states.iter() {
                let mut new_state = state.clone();
                new_state.release(&valves);
                new_states.extend(next_states(&new_state, &valves, openable));
            }

            states = HashSet::from_iter(
                new_states
                    .into_iter()
                    .sorted_by_key(|s: &State| s.total)
                    .rev()
                    .take(states_limit),
            );
        }

        states.iter().map(|s| s.total).max().unwrap()
    }
}

impl Solution for DaySolution {
    fn part1(&self, input: &str) -> Result<Box<dyn Display>> {
        Ok(Box::new(self.solve(
            input,
            30,
            1_000,
            State::my_possible_actions,
        )))
    }

    fn part2(&self, input: &str) -> Result<Box<dyn Display>> {
        Ok(Box::new(self.solve(
            input,
            26,
            2_000,
            State::my_possible_actions_with_elephant,
        )))
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
