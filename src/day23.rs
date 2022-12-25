use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use anyhow::Result;
use aoc::{Runnable, Solution};
use aoc_derive::Runner;
use pathfinding::matrix::directions::DIRECTIONS_8;

#[derive(Runner)]
#[aoc(file = "inputs/day23.txt")]
pub struct DaySolution {
    pub filename: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, parse_display::Display)]
#[display("({x},{y})")]
struct Position {
    x: isize,
    y: isize,
}

impl std::ops::Add<(isize, isize)> for Position {
    type Output = Self;

    fn add(self, (x, y): (isize, isize)) -> Self::Output {
        Self {
            x: self.x + x,
            y: self.y + y,
        }
    }
}

type Map = HashSet<Position>;

trait HasBounds {
    fn bounds(&self) -> (Position, Position);
    fn count_empty(&self) -> usize;
}

#[derive(Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn to_offsets(&self) -> Vec<(isize, isize)> {
        match self {
            // N, NE, NW
            Direction::North => vec![(-1, -1), (0, -1), (1, -1)],
            // S, SE, SW
            Direction::South => vec![(-1, 1), (0, 1), (1, 1)],
            // W, NW, SW
            Direction::West => vec![(-1, -1), (-1, 0), (-1, 1)],
            // E, NE, SE
            Direction::East => vec![(1, -1), (1, 0), (1, 1)],
        }
    }

    fn to_offset(&self) -> (isize, isize) {
        match self {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
            Direction::East => (1, 0),
        }
    }
}

impl HasBounds for Map {
    fn bounds(&self) -> (Position, Position) {
        let (mut min_x, mut min_y) = (isize::MAX, isize::MAX);
        let (mut max_x, mut max_y) = (isize::MIN, isize::MIN);

        for pos in self {
            min_x = min_x.min(pos.x);
            min_y = min_y.min(pos.y);
            max_x = max_x.max(pos.x);
            max_y = max_y.max(pos.y);
        }

        (
            Position { x: min_x, y: min_y },
            Position { x: max_x, y: max_y },
        )
    }

    fn count_empty(&self) -> usize {
        let (min, max) = self.bounds();
        (min.x..=max.x)
            .flat_map(|x| (min.y..=max.y).map(move |y| !self.contains(&Position { x, y })))
            .filter(|&b| b)
            .count()
    }
}

impl DaySolution {
    fn parse(&self, input: &str) -> Map {
        Map::from_iter(input.lines().enumerate().flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                (c == '#').then_some(Position {
                    x: x as isize,
                    y: y as isize,
                })
            })
        }))
    }
}

impl Solution for DaySolution {
    fn part1(&self, input: &str) -> Result<Box<dyn Display>> {
        let mut map = self.parse(input);
        let mut directions = [
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ];

        for _ in 0..10 {
            let mut keep = HashSet::new();
            let mut proposed = HashMap::new();

            for pos in map.iter() {
                let do_not_move = DIRECTIONS_8
                    .iter()
                    .map(|offset| *pos + *offset)
                    .all(|p| !map.contains(&p));

                if do_not_move {
                    keep.insert(*pos);
                    continue;
                }

                let mut is_moved = false;
                for dir in directions.iter() {
                    if dir
                        .to_offsets()
                        .iter()
                        .map(|offset| *pos + *offset)
                        .all(|p| !map.contains(&p))
                    {
                        let new_pos = *pos + dir.to_offset();
                        proposed.entry(new_pos).or_insert_with(Vec::new).push(*pos);

                        is_moved = true;
                        break;
                    }
                }

                if !is_moved {
                    keep.insert(*pos);
                }
            }

            map = keep.clone();
            for (k, v) in proposed.iter() {
                if v.len() == 1 {
                    map.insert(*k);
                } else {
                    map.extend(v.iter());
                }
            }

            directions.rotate_left(1);
        }

        Ok(Box::new(map.count_empty()))
    }

    fn part2(&self, input: &str) -> Result<Box<dyn Display>> {
        let mut map = self.parse(input);
        let mut directions = [
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ];

        let mut round = 0;
        loop {
            let mut keep = HashSet::new();
            let mut proposed = HashMap::new();

            for pos in map.iter() {
                let do_not_move = DIRECTIONS_8
                    .iter()
                    .map(|offset| *pos + *offset)
                    .all(|p| !map.contains(&p));

                if do_not_move {
                    keep.insert(*pos);
                    continue;
                }

                let mut is_moved = false;
                for dir in directions.iter() {
                    if dir
                        .to_offsets()
                        .iter()
                        .map(|offset| *pos + *offset)
                        .all(|p| !map.contains(&p))
                    {
                        let new_pos = *pos + dir.to_offset();
                        proposed.entry(new_pos).or_insert_with(Vec::new).push(*pos);

                        is_moved = true;
                        break;
                    }
                }

                if !is_moved {
                    keep.insert(*pos);
                }
            }

            let prev_map = map.clone();
            map = keep.clone();
            for (k, v) in proposed.iter() {
                if v.len() == 1 {
                    map.insert(*k);
                } else {
                    map.extend(v.iter());
                }
            }

            if prev_map == map {
                break;
            }

            round += 1;
            directions.rotate_left(1);
        }

        Ok(Box::new(round + 1))
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use aoc::day_test;
    use paste::paste;

    day_test!(day23, Part1, "inputs/day23_demo.txt", "110");
    day_test!(day23, Part2, "inputs/day23_demo.txt", "20");
}
