use std::{collections::HashMap, fmt::Display, ops::Add};

use anyhow::Result;
use aoc::{Runnable, Solution};
use aoc_derive::Runner;
use nom::{
    branch::alt,
    character::complete,
    combinator::{map, map_res},
    multi::many1,
    IResult,
};
use num::integer::Roots;

#[derive(Runner)]
#[aoc(file = "inputs/day22.txt")]
pub struct DaySolution {
    pub filename: &'static str,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, parse_display::Display)]
#[display("({x},{y})")]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn x_in_cube(&self, size: isize) -> isize {
        self.x % size
    }

    fn y_in_cube(&self, size: isize) -> isize {
        self.y % size
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, parse_display::Display)]
enum Direction {
    #[display("^")]
    Up,
    #[display(">")]
    Right,
    #[display("V")]
    Down,
    #[display("<")]
    Left,
}

impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn turn_left(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }

    fn offset(&self) -> Position {
        match self {
            Direction::Up => Position::new(0, -1),
            Direction::Right => Position::new(1, 0),
            Direction::Down => Position::new(0, 1),
            Direction::Left => Position::new(-1, 0),
        }
    }

    fn value(&self) -> isize {
        match self {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        }
    }
}

impl Add<Direction> for Position {
    type Output = Self;

    fn add(self, rhs: Direction) -> Self::Output {
        self + rhs.offset()
    }
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Steps(usize),
    TurnLeft,
    TurnRight,
}

impl Instruction {
    fn parse(input: &str) -> IResult<&str, Vec<Self>> {
        let steps = map(
            map_res(complete::digit1, |s: &str| s.parse::<usize>()),
            Instruction::Steps,
        );

        let turn_left = map(complete::char('L'), |_| Instruction::TurnLeft);
        let turn_right = map(complete::char('R'), |_| Instruction::TurnRight);

        let result = many1(alt((steps, turn_left, turn_right)))(input)?;
        Ok(result)
    }
}

type Map = HashMap<Position, char>;

#[derive(Debug)]
struct Person {
    position: Position,
    direction: Direction,
}

impl Person {
    fn new(position: Position) -> Self {
        Self {
            position,
            direction: Direction::Right,
        }
    }

    fn turn(&mut self, direction: char) {
        self.direction = match direction {
            'L' => self.direction.turn_left(),
            'R' => self.direction.turn_right(),
            _ => unreachable!(),
        };
    }

    fn walk(&mut self, map: &Map, steps: usize) {
        for _ in 0..steps {
            let new_position = self.position + self.direction;

            match map.get(&new_position) {
                Some('.') => self.position = new_position,
                Some('#') => break,
                None => match self.direction {
                    Direction::Down => {
                        let new_position = map
                            .keys()
                            .filter(|p| p.x == self.position.x)
                            .min_by_key(|p| p.y)
                            .unwrap();

                        if new_position != &self.position && map.get(new_position) == Some(&'.') {
                            self.position = *new_position;
                        }
                    }
                    Direction::Right => {
                        let new_position = map
                            .keys()
                            .filter(|p| p.y == self.position.y)
                            .min_by_key(|p| p.x)
                            .unwrap();

                        if new_position != &self.position && map.get(new_position) == Some(&'.') {
                            self.position = *new_position;
                        }
                    }
                    Direction::Up => {
                        let new_position = map
                            .keys()
                            .filter(|p| p.x == self.position.x)
                            .max_by_key(|p| p.y)
                            .unwrap();

                        if new_position != &self.position && map.get(new_position) == Some(&'.') {
                            self.position = *new_position;
                        }
                    }
                    Direction::Left => {
                        let new_position = map
                            .keys()
                            .filter(|p| p.y == self.position.y)
                            .max_by_key(|p| p.x)
                            .unwrap();

                        if new_position != &self.position && map.get(new_position) == Some(&'.') {
                            self.position = *new_position;
                        }
                    }
                },
                _ => unreachable!(),
            }
        }
    }

    fn follow(&mut self, map: &Map, instruction: &Instruction) {
        match instruction {
            Instruction::Steps(steps) => self.walk(map, *steps),
            Instruction::TurnLeft => self.turn('L'),
            Instruction::TurnRight => self.turn('R'),
        }
    }

    fn follow_cube(&mut self, map: &Map, instruction: &Instruction, cube_size: isize) {
        match instruction {
            Instruction::Steps(steps) => self.walk_cube(map, *steps, cube_size),
            Instruction::TurnLeft => self.turn('L'),
            Instruction::TurnRight => self.turn('R'),
        }
    }

    fn walk_cube(&mut self, map: &Map, steps: usize, size: isize) {
        for _ in 0..steps {
            let new_position = self.position + self.direction;
            match map.get(&new_position) {
                Some('.') => self.position = new_position,
                Some('#') => break,
                None => match self.direction {
                    Direction::Up => {
                        if self.position.x / size == 0 {
                            let new_position =
                                Position::new(size, self.position.x_in_cube(size) + size);

                            if map.get(&new_position) == Some(&'.') {
                                self.position = new_position;
                                self.direction = Direction::Right;
                            }
                        } else if self.position.x / size == 1 {
                            let new_position =
                                Position::new(0, self.position.x_in_cube(size) + 3 * size);

                            if map.get(&new_position) == Some(&'.') {
                                self.position = new_position;
                                self.direction = Direction::Right;
                            }
                        } else if self.position.x / size == 2 {
                            let new_position =
                                Position::new(self.position.x_in_cube(size), 4 * size - 1);

                            if map.get(&new_position) == Some(&'.') {
                                self.position = new_position;
                            }
                        }
                    }
                    Direction::Down => {
                        if self.position.x / size == 0 {
                            let new_position = Position::new(size * 2 + self.position.x, 0);

                            if map.get(&new_position) == Some(&'.') {
                                self.position = new_position;
                            }
                        } else if self.position.x / size == 1 {
                            let new_position =
                                Position::new(size - 1, self.position.x_in_cube(size) + 3 * size);
                            if map.get(&new_position) == Some(&'.') {
                                self.position = new_position;
                                self.direction = Direction::Left;
                            }
                        } else if self.position.x / size == 2 {
                            let new_position =
                                Position::new(size * 2 - 1, self.position.x_in_cube(size) + size);

                            if map.get(&new_position) == Some(&'.') {
                                self.position = new_position;
                                self.direction = Direction::Left;
                            }
                        }
                    }
                    Direction::Right => {
                        if self.position.y / size == 0 {
                            let new_position = Position::new(
                                size * 2 - 1,
                                size * 3 - self.position.y_in_cube(size) - 1,
                            );
                            if map.get(&new_position) == Some(&'.') {
                                self.position = new_position;
                                self.direction = Direction::Left;
                            }
                        } else if self.position.y / size == 1 {
                            let new_position =
                                Position::new(size * 2 + self.position.y_in_cube(size), size - 1);

                            if map.get(&new_position) == Some(&'.') {
                                self.position = new_position;
                                self.direction = Direction::Up;
                            }
                        } else if self.position.y / size == 2 {
                            let new_position = Position::new(
                                size * 3 - 1,
                                size - self.position.y_in_cube(size) - 1,
                            );

                            if map.get(&new_position) == Some(&'.') {
                                self.position = new_position;
                                self.direction = Direction::Left;
                            }
                        } else if self.position.y / size == 3 {
                            let new_position =
                                Position::new(size + self.position.y_in_cube(size), size * 3 - 1);

                            if map.get(&new_position) == Some(&'.') {
                                self.position = new_position;
                                self.direction = Direction::Up;
                            }
                        }
                    }
                    Direction::Left => {
                        if self.position.y / size == 0 {
                            let new_position =
                                Position::new(0, size * 3 - self.position.y_in_cube(size) - 1);
                            if map.get(&new_position) == Some(&'.') {
                                self.position = new_position;
                                self.direction = Direction::Right;
                            }
                        } else if self.position.y / size == 1 {
                            let new_position =
                                Position::new(self.position.y_in_cube(size), size * 2);

                            if map.get(&new_position) == Some(&'.') {
                                self.position = new_position;
                                self.direction = Direction::Down;
                            }
                        } else if self.position.y / size == 2 {
                            let new_position =
                                Position::new(size, size - self.position.y_in_cube(size) - 1);
                            if map.get(&new_position) == Some(&'.') {
                                self.position = new_position;
                                self.direction = Direction::Right;
                            }
                        } else if self.position.y / size == 3 {
                            let new_position =
                                Position::new(self.position.y_in_cube(size) + size, 0);

                            if map.get(&new_position) == Some(&'.') {
                                self.position = new_position;
                                self.direction = Direction::Down;
                            }
                        }
                    }
                },
                _ => unreachable!(),
            }
        }
    }

    fn password(&self) -> isize {
        (self.position.y + 1) * 1000 + (self.position.x + 1) * 4 + self.direction.value()
    }
}

impl DaySolution {
    fn parse(&self, input: &str) -> Result<(Map, Vec<Instruction>)> {
        let (map, instructions) = input.split_once("\n\n").unwrap();
        let (_, instructions) = Instruction::parse(instructions).unwrap();

        let map = Map::from_iter(map.lines().enumerate().flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c == ' ' {
                    return None;
                }

                Some((Position::new(x as isize, y as isize), c))
            })
        }));

        Ok((map, instructions))
    }
}

impl Solution for DaySolution {
    fn part1(&self, input: &str) -> Result<Box<dyn Display>> {
        let (map, instructions) = self.parse(input)?;

        let position = map.keys().min_by_key(|p| (p.y, p.x)).unwrap();
        let mut person = Person::new(*position);

        for instruction in instructions {
            person.follow(&map, &instruction);
        }

        Ok(Box::new(person.password()))
    }

    fn part2(&self, input: &str) -> Result<Box<dyn Display>> {
        let (map, instructions) = self.parse(input)?;
        let size = (map.values().count() / 6).sqrt() as isize;

        let position = map.keys().min_by_key(|p| (p.y, p.x)).unwrap();
        let mut person = Person::new(*position);

        for instruction in instructions {
            person.follow_cube(&map, &instruction, size);
        }

        Ok(Box::new(person.password()))
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use aoc::day_test;
    use paste::paste;

    day_test!(day22, Part1, "inputs/day22_demo.txt", "6032");
    day_test!(day22, Part2, "inputs/day22.txt", "129339");
}
