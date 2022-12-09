use aoc::{count, runners, Runnable};

mod matrix;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;

fn main() {
    let day = std::env::args()
        .nth(1)
        .and_then(|v| v.parse::<usize>().ok());

    runners!(day01, day02, day03, day04, day05, day06, day07, day08, day09)
        .iter()
        .enumerate()
        .filter(|(d, _)| day.map(|day| day - 1 == *d).unwrap_or(true))
        .for_each(|(day, runner)| {
            println!("Day #{:02}\n{}", day + 1, "-".repeat(32));

            aoc::SolutionPart::iter().for_each(|part| match runner.run(part) {
                Ok(result) => println!("{}: {}", part, result),
                Err(err) => println!("{}: {}", part, err),
            });

            println!();
        });
}
