use std::{fmt::Display, process::Command, str::FromStr};

use anyhow::Result;
use aoc::{Runnable, Solution};
use aoc_derive::Runner;

#[derive(Runner)]
#[aoc(file = "inputs/day21.txt")]
pub struct DaySolution {
    pub filename: &'static str,
}

impl DaySolution {}

impl Solution for DaySolution {
    fn part1(&self, input: &str) -> Result<Box<dyn Display>> {
        let mut js_lines = vec![];

        for line in input.lines() {
            let (func, expr) = line.split_once(": ").unwrap();

            if let Ok(expr) = u32::from_str(expr) {
                js_lines.push(format!("function {}() {{ return {}; }}", func, expr));
                continue;
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

            js_lines.push(format!("function {}() {{ return {}; }}", func, expr));
        }

        js_lines.push("console.log(root());".to_string());
        let js_code = js_lines.join("\n");

        let result = Command::new("node")
            .arg("-e")
            .arg(js_code)
            .output()
            .map(|output| String::from_utf8(output.stdout))
            .map_err(|err| anyhow::anyhow!("Failed to run node: {}", err))??;

        Ok(Box::new(result))
    }

    fn part2(&self, input: &str) -> Result<Box<dyn Display>> {
        let mut js_lines = vec![];

        for line in input.lines() {
            let (func, expr) = line.split_once(": ").unwrap();

            if func == "humn" {
                js_lines.push(format!("function {}() {{ return 'x'; }}", func));
                continue;
            }

            if let Ok(expr) = u32::from_str(expr) {
                js_lines.push(format!("function {}() {{ return {}; }}", func, expr));
                continue;
            }

            let expr = expr
                .split(' ')
                .map(|p| match (u32::from_str(p), p) {
                    (Ok(num), _) => num.to_string(),
                    (_, "+" | "-" | "*" | "/") => p.to_string(),
                    _ => format!("{}()", p),
                })
                .collect::<Vec<_>>();

            let a = expr[0].clone();
            let op = expr[1].clone();
            let b = expr[2].clone();

            let expr = if func == "root" {
                format!("return `${{{}}} = ${{{}}}`", a, b)
            } else {
                format!(
                    "
    let a = {}; let b = {};
    if (a !== 'x' && typeof a !== 'number') a = '(' + a + ')';
    if (b !== 'x' && typeof b !== 'number') b = '(' + b + ')';

    return `${{a}} {} ${{b}}`",
                    a, b, op
                )
            };

            js_lines.push(format!("function {}() {{ {} }}", func, expr));
        }

        js_lines.push(
            r#"
function simplify(expr) {
  return expr.replace(/\((\d+)\s+([+\-*\/])\s+(\d+)\)/g, (_, a, op, b) =>
    eval(`${a} ${op} ${b}`)
  );
}

let expr = root();
while (true) {
  let newExpr = simplify(expr);
  if (newExpr === expr) break;
  expr = newExpr;
}

console.log(expr);"#
                .to_string(),
        );
        let js_code = js_lines.join("\n");

        let result = Command::new("node")
            .arg("-e")
            .arg(js_code)
            .output()
            .map(|output| String::from_utf8(output.stdout))
            .map_err(|err| anyhow::anyhow!("Failed to run node: {}", err))??;

        Ok(Box::new(result))
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use aoc::day_test;
    use paste::paste;

    day_test!(day21, Part1, "inputs/day21_demo.txt", "152\n");
    day_test!(
        day21,
        Part2,
        "inputs/day21_demo.txt",
        "(4 + (2 * (x - 3))) / 4 = 30 * 5\n"
    );
}
