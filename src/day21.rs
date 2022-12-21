use std::{fmt::Display, process::Command, str::FromStr};

use anyhow::Result;
use aoc::{Runnable, Solution};
use aoc_derive::Runner;
use itertools::Itertools;
use nom::{
    branch::alt,
    character::complete,
    combinator::{map, map_res},
    sequence::{delimited, tuple},
    IResult,
};

#[derive(Runner)]
#[aoc(file = "inputs/day21.txt")]
pub struct DaySolution {
    pub filename: &'static str,
}

impl DaySolution {}

#[derive(Debug)]
enum Operand {
    Num(i64),
    X,
    Expr(Box<Expr>),
}

impl Operand {
    fn eval(&self) -> i64 {
        match self {
            Operand::Num(n) => *n,
            Operand::Expr(expr) => expr.eval(),
            _ => unreachable!(),
        }
    }

    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, operand) = alt((
            map(
                delimited(complete::char('('), Expr::parse, complete::char(')')),
                |expr| Self::Expr(Box::new(expr)),
            ),
            map(complete::char('x'), |_| Self::X),
            map(
                map_res(complete::digit1, |s: &str| s.parse::<i64>()),
                Self::Num,
            ),
        ))(input)?;
        Ok((input, operand))
    }
}

#[derive(Debug)]
struct Expr {
    left: Operand,
    op: char,
    right: Operand,
}

impl Expr {
    fn eval(&self) -> i64 {
        match self.op {
            '+' => self.left.eval() + self.right.eval(),
            '-' => self.left.eval() - self.right.eval(),
            '*' => self.left.eval() * self.right.eval(),
            '/' => self.left.eval() / self.right.eval(),
            _ => unreachable!(),
        }
    }

    fn apply_op(&self, x: i64) -> i64 {
        match (&self.left, &self.right) {
            (Operand::Num(n), _) => match self.op {
                '+' => x - n,
                '-' => n - x,
                '*' => x / n,
                '/' => n / x,
                _ => x,
            },
            (_, Operand::Num(n)) => match self.op {
                '+' => x - n,
                '-' => x + n,
                '*' => x / n,
                '/' => x * n,
                _ => x,
            },
            _ => x,
        }
    }

    fn solve(&self, x: i64) -> i64 {
        let mut current = self;
        let mut x = x;

        loop {
            x = current.apply_op(x);
            current = match (&current.left, &current.right) {
                (_, Operand::X) => {
                    break;
                }
                (Operand::X, _) => {
                    break;
                }
                (_, Operand::Expr(expr)) => expr,
                (Operand::Expr(expr), _) => expr,
                _ => unreachable!(),
            };
        }

        x
    }

    fn parse(input: &str) -> IResult<&str, Expr> {
        let (input, (left, op, right)) =
            tuple((Operand::parse, complete::one_of("+-*/"), Operand::parse))(input)?;

        Ok((input, Expr { left, op, right }))
    }
}

impl Solution for DaySolution {
    fn part1(&self, input: &str) -> Result<Box<dyn Display>> {
        let js_lines = input
            .lines()
            .map(|line| {
                let (func, expr) = line.split_once(": ").unwrap();

                if let Ok(expr) = u32::from_str(expr) {
                    return format!("function {}() {{ return {} }}", func, expr);
                }

                let expr = expr
                    .split(' ')
                    .map(|p| match (u32::from_str(p), p) {
                        (Ok(num), _) => num.to_string(),
                        (_, "+" | "-" | "*" | "/") => p.to_string(),
                        _ => format!("{}()", p),
                    })
                    .collect::<Vec<_>>()
                    .join(" ");

                format!("function {}() {{ return {} }}", func, expr)
            })
            .merge(vec!["console.log(root());".to_string()])
            .collect::<Vec<_>>();

        let result = Command::new("node")
            .arg("-e")
            .arg(js_lines.join("\n"))
            .output()
            .map(|output| String::from_utf8(output.stdout))
            .map_err(|err| anyhow::anyhow!("Failed to run node: {}", err))??
            .trim()
            .to_owned();

        Ok(Box::new(result))
    }

    fn part2(&self, input: &str) -> Result<Box<dyn Display>> {
        let js_lines = input
            .lines()
            .map(|line| {
                let (func, expr) = line.split_once(": ").unwrap();

                if func == "humn" {
                    return format!("function {}() {{ return 'x'; }}", func);
                }

                if let Ok(expr) = u32::from_str(expr) {
                    return format!("function {}() {{ return {}; }}", func, expr);
                }

                let expr = expr
                    .split(' ')
                    .map(|p| match (u32::from_str(p), p) {
                        (Ok(num), _) => num.to_string(),
                        (_, "+" | "-" | "*" | "/") => p.to_string(),
                        _ => format!("{}()", p),
                    })
                    .collect::<Vec<_>>();

                let (a, op, b) = (expr[0].clone(), expr[1].clone(), expr[2].clone());

                if func == "root" {
                    return format!("function {}() {{ return `${{{}}} = ${{{}}}` }}", func, a, b);
                }

                let expr =  format!("
                    let a = {}; let b = {};
                    if (a !== 'x' && typeof a !== 'number') a = '(' + a + ')';
                    if (b !== 'x' && typeof b !== 'number') b = '(' + b + ')';
                
                    return `${{a}} {} ${{b}}`",
                    a, b, op);

                format!("function {}() {{ {} }}", func, expr)
            })
            .merge(vec![
                r#"
                let expr = root();
                while (true) {
                let newExpr = expr.replace(/\((\d+)\s+([+\-*\/])\s+(\d+)\)/g, (_, a, op, b) => eval(`${a} ${op} ${b}`));
                if (newExpr === expr) break;
                expr = newExpr;
                }
                
                console.log(expr);
                "#.to_string()
            ])
            .collect::<Vec<_>>();

        let result = Command::new("node")
            .arg("-e")
            .arg(js_lines.join("\n"))
            .output()
            .map(|output| String::from_utf8(output.stdout))
            .map_err(|err| anyhow::anyhow!("Failed to run node: {}", err))??
            .replace(' ', "");

        let (first, second) = result.split_once('=').unwrap();
        let (expr, value) = if first.contains('x') {
            (first, second)
        } else {
            (second, first)
        };

        let (_, ast) = Expr::parse(expr).unwrap();
        let (_, value) = Expr::parse(value).unwrap();

        Ok(Box::new(ast.solve(value.eval())))
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use aoc::day_test;
    use paste::paste;

    day_test!(day21, Part1, "inputs/day21_demo.txt", "152");
    day_test!(day21, Part2, "inputs/day21_demo.txt", "301");
}
