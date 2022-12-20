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
enum Action {
    Open(String),
    Move(String, String),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct State {
    opened: Vec<String>,
    your: String,
    total: i32,
    elephant: Option<String>,
    history: Vec<String>,

    your_action: Option<Action>,
    elephant_action: Option<Action>,
}

impl State {
    fn new(current: impl ToString) -> Self {
        State {
            opened: vec![],
            your: current.to_string(),
            total: 0,
            elephant: None,
            history: vec![],
            your_action: None,
            elephant_action: None,
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
        let valve = valves.get(&self.your).unwrap();

        // Open current
        if !self.opened.contains(&self.your) && valve.rate > 0 {
            let mut new_state = self.clone();

            new_state.opened.push(self.your.clone());
            new_state.your_action = Some(Action::Open(self.your.clone()));

            actions.push(new_state);
        }

        // Only release if all opened
        if possible_open == self.opened.len() {
            let mut state = self.clone();
            state.your_action = None;
            state.elephant_action = None;

            actions.push(state);
        }

        // Move to neighbour
        for neighbour in valve.connections.iter() {
            let mut neighbour_state = self.clone();
            neighbour_state.your = neighbour.clone();
            neighbour_state.your_action = Some(Action::Move(self.your.clone(), neighbour.clone()));

            actions.push(neighbour_state);
        }

        actions
    }

    fn my_possible_actions_with_elephant(
        &mut self,
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
                let action = my_action.your_action.clone().unwrap();

                match action {
                    Action::Open(opened) => {
                        for neighbour in valves.get(&self.your).unwrap().connections.iter() {
                            if neighbour == &opened {
                                continue;
                            }

                            let mut neighbour_state = my_action.clone();
                            neighbour_state.elephant_action =
                                Some(Action::Move(self.your.clone(), neighbour.clone()));

                            neighbour_state.elephant = Some(neighbour.clone());
                            actions.push(neighbour_state);
                        }
                    }
                    Action::Move(from, to) => {
                        // open current
                        let valve = valves.get(&self.your).unwrap();

                        if !my_action.opened.contains(&self.your) && valve.rate > 0 {
                            let mut new_state = my_action.clone();
                            new_state.elephant_action = Some(Action::Open(self.your.clone()));
                            new_state.opened.push(self.your.clone());

                            actions.push(new_state);
                        }
                        // move

                        for neighbour in valves.get(&from).unwrap().connections.iter() {
                            if neighbour == &to {
                                continue;
                            }

                            let mut neighbour_state = my_action.clone();
                            neighbour_state.elephant_action =
                                Some(Action::Move(from.clone(), neighbour.clone()));

                            neighbour_state.elephant = Some(neighbour.clone());
                            actions.push(neighbour_state);
                        }
                    }
                }
            } else {
                let elephant = self.elephant.clone().unwrap();
                let elephant_valve = valves.get(&elephant).unwrap();

                // open
                if elephant_valve.rate > 0 && !self.opened.contains(&elephant) {
                    if let Some(Action::Open(opening)) = &my_action.your_action {
                        if opening != &elephant {
                            let mut new_state = my_action.clone();
                            new_state.elephant_action = Some(Action::Open(elephant.clone()));
                            new_state.opened.push(elephant.clone());

                            actions.push(new_state);
                        }
                    } else {
                        let mut new_state = my_action.clone();
                        new_state.elephant_action = Some(Action::Open(elephant.clone()));
                        new_state.opened.push(elephant.clone());

                        actions.push(new_state);
                    }
                }

                // let action = my_action.your_action.clone().unwrap();

                // move
                for neighbour in valves.get(&elephant).unwrap().connections.iter() {
                    if neighbour == &self.your {
                        continue;
                    }

                    let mut neighbour_state = my_action.clone();
                    neighbour_state.elephant_action =
                        Some(Action::Move(elephant.clone(), neighbour.clone()));

                    neighbour_state.elephant = Some(neighbour.clone());
                    actions.push(neighbour_state);
                }
            }
        }

        actions
    }
}

impl Solution for DaySolution {
    fn part1(&self, input: &str) -> Result<Box<dyn Display>> {
        let valves = self.parse(input);

        let openable = valves
            .iter()
            .filter(|(_, valve)| valve.rate > 0)
            .map(|(id, _)| id)
            .count();

        let mut states = HashSet::new();
        let state = State::new("AA");
        states.insert(state);

        for _ in 0..30 {
            let mut new_states = HashSet::new();

            for state in states.iter() {
                let mut new_state = state.clone();
                new_state.release(&valves);
                new_states.extend(new_state.my_possible_actions(&valves, openable));
            }

            states = HashSet::from_iter(
                new_states
                    .into_iter()
                    .sorted_by_key(|s: &State| s.total)
                    .rev()
                    .take(1000),
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
            .count();

        let mut states = HashSet::new();
        let state = State::new("AA");
        states.insert(state);

        for _ in 0..26 {
            let mut new_states = HashSet::new();

            for state in states.iter() {
                let mut new_state = state.clone();

                new_state.release(&valves);
                new_states.extend(new_state.my_possible_actions_with_elephant(&valves, openable));
            }

            states = HashSet::from_iter(
                new_states
                    .into_iter()
                    .sorted_by_key(|s: &State| s.total)
                    .rev()
                    .take(10_000),
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
