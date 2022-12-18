use std::{collections::HashSet, fmt::Display};

use anyhow::Result;
use aoc::{Runnable, Solution};
use aoc_derive::Runner;

#[derive(Runner)]
#[aoc(file = "inputs/day17.txt")]
pub struct DaySolution {
    pub filename: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
struct Rock {
    points: Vec<Point>,
}

impl Rock {
    fn new(shape: &str) -> Self {
        let mut points = vec![];
        for (y, line) in shape.lines().rev().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    points.push(Point { x, y });
                }
            }
        }

        Rock { points }
    }

    fn all_available() -> Vec<Rock> {
        vec![
            Self::new("####"),
            Self::new(".#.\n###\n.#."),
            Self::new("..#\n..#\n###"),
            Self::new("#\n#\n#\n#"),
            Self::new("##\n##"),
        ]
    }

    fn move_left(&mut self, map: &HashSet<Point>) -> bool {
        if self
            .points
            .iter()
            .any(|p| p.x == 0 || map.contains(&Point { x: p.x - 1, y: p.y }))
        {
            return false;
        }
        self.points.iter_mut().for_each(|p| p.x -= 1);

        true
    }

    fn move_right(&mut self, map: &HashSet<Point>) -> bool {
        if self
            .points
            .iter()
            .any(|p| p.x == 6 || map.contains(&Point { x: p.x + 1, y: p.y }))
        {
            return false;
        }
        self.points.iter_mut().for_each(|p| p.x += 1);

        true
    }

    fn top(&self) -> usize {
        self.points.iter().map(|p| p.y).min().unwrap()
    }

    fn bottom(&self) -> usize {
        self.points.iter().map(|p| p.y).max().unwrap()
    }

    fn height(&self) -> usize {
        self.bottom() - self.top() + 1
    }

    fn move_by(&mut self, x: usize, y: usize) {
        self.points.iter_mut().for_each(|p| {
            p.x += x;
            p.y += y;
        });
    }

    // fn moved_by(directions: &[char]) -> Self {
    //     // let rock.move_by(2, 0);
    //     // for c in directions.chars() {
    //     //     match c {
    //     //         'L' => self.move_left(&HashSet::new()),
    //     //         'R' => self.move_right(&HashSet::new()),
    //     //         'D' => self.down(&HashSet::new()),
    //     //         _ => {}
    //     //     }
    //     // }
    // }

    fn down(&mut self, map: &HashSet<Point>) -> bool {
        if self
            .points
            .iter()
            .any(|p| p.y == 0 || map.contains(&Point { x: p.x, y: p.y - 1 }))
        {
            return false;
        }

        self.points.iter_mut().for_each(|p| p.y -= 1);
        true
    }
}

impl DaySolution {
    fn debug(&self, map: &HashSet<Point>, max_y: usize, current: Option<&Rock>) {
        for y in (0..=max_y).rev() {
            print!("|");
            for x in 0..=6 {
                if map.contains(&Point { x, y }) {
                    print!("#");
                } else {
                    if let Some(rock) = current {
                        if rock.points.contains(&Point { x, y }) {
                            print!("@");
                            continue;
                        }
                    }
                    print!(".");
                }
            }
            print!("|");
            println!();
        }

        println!("+-------+");
    }
}

impl Solution for DaySolution {
    fn part1(&self, input: &str) -> Result<Box<dyn Display>> {
        let rocks = Rock::all_available();
        let mut map = HashSet::<Point>::new();
        let mut y = 3;
        let gas_diretions = input.chars().collect::<Vec<_>>();
        let mut rock_index = 0;
        let mut direction_index = 0;

        // let now = std::time::Instant::now();

        for _ in 0..5000 {
            let mut rock = rocks[rock_index % rocks.len()].clone();
            rock.move_by(2, y);
            // println!("Rock: #{i}");
            // self.debug(&map, y + rock.height() - 1, Some(&rock));

            // if i % 1_000_000i64 == 0 {
            //     println!("i: {}", i);

            //     let elapsed = now.elapsed();
            //     println!("Elapsed: {:?}", elapsed);
            // }

            loop {
                let direction = gas_diretions[direction_index % gas_diretions.len()];
                if direction == '>' {
                    _ = !rock.move_right(&map);
                } else {
                    _ = !rock.move_left(&map);
                }

                // self.debug(&map, y + rock.height() - 1, Some(&rock));

                direction_index += 1;
                if !rock.down(&map) {
                    break;
                }
            }

            map.extend(rock.points.iter().cloned());

            // println!();
            // println!();

            rock_index += 1;
            y = y.max(rock.top() + rock.height() + 3);
        }

        // dbg!()
        self.debug(&map, y, None);
        let max_y = map.iter().map(|p| p.y).max().unwrap() + 1;

        dbg!(max_y);

        Ok(Box::new(max_y))
    }

    fn part2(&self, _input: &str) -> Result<Box<dyn Display>> {
        Ok(Box::new(0))
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use aoc::day_test;
    use paste::paste;

    day_test!(day17, Part1, "inputs/day17_demo.txt", "3068");
    day_test!(day17, Part2, "inputs/day17_demo.txt", "unknown");
}
