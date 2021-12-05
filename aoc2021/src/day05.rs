use std::collections::HashMap;
use std::fmt::Write;
use std::iter;

use anyhow::{Context, Result};

const INPUT: &str = include_str!("../input/day05.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;
    writeln!(res, "part 2: {}", part2(INPUT)?)?;

    Ok(res)
}

fn part1(input: &str) -> Result<usize> {
    let lines = input
        .lines()
        .map(str::parse::<Line>)
        .collect::<Result<Vec<_>>>()?;

    let mut grid = HashMap::new();

    lines
        .iter()
        .filter(|l| l.is_horizontal() || l.is_vertical())
        .for_each(|l| {
            for cell in l.cells() {
                *grid.entry(cell).or_insert(0) += 1;
            }
        });

    Ok(grid.into_values().filter(|c| *c > 1).count())
}

fn part2(input: &str) -> Result<usize> {
    let lines = input
        .lines()
        .map(str::parse::<Line>)
        .collect::<Result<Vec<_>>>()?;

    let mut grid = HashMap::new();

    lines.iter().for_each(|l| {
        for cell in l.cells() {
            *grid.entry(cell).or_insert(0) += 1;
        }
    });

    Ok(grid.into_values().filter(|c| *c > 1).count())
}

#[derive(Debug)]
struct Line {
    from: (usize, usize),
    to: (usize, usize),
}

impl Line {
    fn is_horizontal(&self) -> bool {
        self.to.1 == self.from.1
    }

    fn is_vertical(&self) -> bool {
        self.to.0 == self.from.0
    }

    fn cells(&self) -> Box<dyn Iterator<Item = (usize, usize)>> {
        fn inclusive_range_any_order(a: usize, b: usize) -> Box<dyn Iterator<Item = usize>> {
            if a < b {
                Box::new(a..=b) as Box<dyn Iterator<Item = _>>
            } else {
                Box::new((b..=a).rev())
            }
        }

        if self.is_horizontal() {
            Box::new(
                inclusive_range_any_order(self.from.0, self.to.0).zip(iter::repeat(self.from.1)),
            ) as Box<dyn Iterator<Item = _>>
        } else if self.is_vertical() {
            Box::new(
                iter::repeat(self.from.0).zip(inclusive_range_any_order(self.from.1, self.to.1)),
            )
        } else {
            let x_range = inclusive_range_any_order(self.from.0, self.to.0);
            let y_range = inclusive_range_any_order(self.from.1, self.to.1);
            Box::new(x_range.zip(y_range))
        }
    }
}

impl std::str::FromStr for Line {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (from, to) = s
            .split_once(" -> ")
            .context("couldn't parse line: missing ` -> `")?;

        let from = {
            let (x, y) = from
                .split_once(',')
                .context("couldn't parse line origin: missing `,`")?;
            (x.parse()?, y.parse()?)
        };

        let to = {
            let (x, y) = to
                .split_once(',')
                .context("couldn't parse line end: missing `,`")?;
            (x.parse()?, y.parse()?)
        };

        Ok(Line { from, to })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED: &str = include_str!("../input/day05_provided.txt");

    #[test]
    fn part1_provided() {
        assert_eq!(part1(PROVIDED).unwrap(), 5);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 4745);
    }

    #[test]
    fn part2_provided() {
        assert_eq!(part2(PROVIDED).unwrap(), 12);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT).unwrap(), 18442);
    }
}
