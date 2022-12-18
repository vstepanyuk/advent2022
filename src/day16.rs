use std::{collections::HashMap, fmt::Display};

use anyhow::Result;
use aoc::{Runnable, Solution};
use aoc_derive::Runner;
use nom::{
    self, branch::alt, bytes::complete::tag, bytes::complete::take, character::complete::digit1,
    combinator::map_res, multi::separated_list1, sequence::tuple, IResult,
};
use pathfinding::prelude::dijkstra;

#[derive(Runner)]
#[aoc(file = "inputs/day16.txt")]
pub struct DaySolution {
    pub filename: &'static str,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Valve<'a> {
    id: &'a str,
    rate: i32,
    leads_to: Vec<&'a str>,
}

impl Valve<'_> {
    fn parse(i: &str) -> IResult<&str, Valve> {
        let (_, (_, id, _, rate, _, leads_to)) = tuple((
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

        Ok((i, Valve { id, rate, leads_to }))
    }
}

impl DaySolution {
    fn parse<'a>(&self, input: &'a str) -> HashMap<&'a str, Valve<'a>> {
        HashMap::from_iter(input.lines().map(|line| {
            let (_, valve) = Valve::parse(line).unwrap();

            (valve.id, valve)
        }))
    }
}

impl Solution for DaySolution {
    fn part1(&self, input: &str) -> Result<Box<dyn Display>> {
        let valves = self.parse(input);

        let max_rate = valves
            .values()
            .map(|valve| valve.rate)
            .max()
            .unwrap_or_default();

        dbg!(max_rate);

        let result = dijkstra(
            &("AA".to_owned(), 30),
            |(current, minutes)| {
                dbg!(current, minutes);

                let mut result = vec![];

                for lead in valves.get(current.as_str()).unwrap().leads_to.iter() {
                    let valve = valves.get(lead).unwrap();
                    let rate = valve.rate;

                    result.push(((lead.to_string(), *minutes - 2), max_rate - rate));
                }

                result
            },
            |&(_, minutes)| minutes == 0,
        );

        println!("{:?}", result);
        println!("{:?}", valves);

        Err(anyhow::anyhow!("Not implemented"))
    }

    fn part2(&self, _input: &str) -> Result<Box<dyn Display>> {
        Err(anyhow::anyhow!("Not implemented"))
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use aoc::day_test;
    use paste::paste;

    day_test!(day16, Part1, "inputs/day16_demo.txt", "unknown");
    day_test!(day16, Part2, "inputs/day16_demo.txt", "unknown");
}
