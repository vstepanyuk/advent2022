use aoc::{count, runners, Runnable};

mod day01;
mod day02;
mod day03;

fn main() {
    runners!(day01, day02, day03)
        .iter()
        .enumerate()
        .for_each(|(day, runner)| {
            println!("Day {}\n{}", day + 1, "-".repeat(32));

            aoc::SolutionPart::iter().for_each(|part| match runner.run(part) {
                Ok(result) => println!("{}: {}", part, result),
                Err(err) => println!("{}: {}", part, err),
            });

            println!();
        });
}
