use anyhow::Result;
use std::fmt::Display;

mod macros {
    #[macro_export]
    macro_rules! day_test {
        ($day: ident, $part:ident, $filename:literal, $result: literal) => {
            paste! {
                #[test]
                fn [<test_ $day _ $part:lower>]() -> anyhow::Result<()> {
                    let day = $day::DaySolution {
                        filename: $filename,
                    };
                    assert_eq!(day.run(aoc::SolutionPart::$part)?.to_string(), $result);
                    Ok(())
                }
            }
        };
    }

    #[macro_export]
    macro_rules! count {
        () => (0usize);
        ( $x:tt $($xs:tt)* ) => (1usize + count!($($xs)*));
    }

    #[macro_export]
    macro_rules! runners {
        ($( $mod_name:ident ), *) => { [$( Box::new($mod_name::DaySolution::default()),)*] as [Box<dyn Runnable>; count!($($mod_name)*)] };
    }
}
pub trait Boxed {
    fn boxed(self) -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(self)
    }

    fn result(self) -> Result<Box<Self>>
    where
        Self: Sized,
    {
        Ok(Box::new(self))
    }
}

impl<'a, T> Boxed for T where T: 'a {}

#[derive(Clone, Copy)]
pub enum SolutionPart {
    Part1,
    Part2,
}

impl SolutionPart {
    pub fn iter() -> impl Iterator<Item = SolutionPart> {
        [SolutionPart::Part1, SolutionPart::Part2].iter().copied()
    }
}

impl Display for SolutionPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SolutionPart::Part1 => write!(f, "Part 1"),
            SolutionPart::Part2 => write!(f, "Part 2"),
        }
    }
}

pub trait Runnable: Solution {
    fn filename(&self) -> String;

    fn run(&self, part: SolutionPart) -> Result<Box<dyn Display>> {
        let filename = self.filename();
        let input = std::fs::read_to_string(filename.as_str())
            .map_err(|_| anyhow::anyhow!("Input file {filename} not found"))?;

        Ok(Box::new(match part {
            SolutionPart::Part1 => self.part1(&input)?,
            SolutionPart::Part2 => self.part2(&input)?,
        }))
    }
}

pub trait Solution {
    fn part1(&self, _input: &str) -> Result<Box<dyn Display>> {
        Err(anyhow::anyhow!("not implemented"))
    }

    fn part2(&self, _input: &str) -> Result<Box<dyn Display>> {
        Err(anyhow::anyhow!("not implemented"))
    }
}
