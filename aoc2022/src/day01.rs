use std::fmt::Write;

use anyhow::{Context, Result};

const INPUT: &str = include_str!("../input/day01.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;

    Ok(res)
}

fn part1(input: &str) -> Result<u64> {
    let inventories = input
        .split("\n\n")
        .map(str::parse::<Inventory>)
        .collect::<Result<Vec<_>>>()?;

    inventories
        .iter()
        .map(Inventory::total_calories)
        .max()
        .context("inventory list was empty")
}

struct Inventory(Vec<u64>);

impl std::str::FromStr for Inventory {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Inventory(
            s.lines()
                .map(|line| line.parse::<u64>().map_err(anyhow::Error::new))
                .collect::<Result<Vec<_>>>()?,
        ))
    }
}

impl Inventory {
    fn total_calories(&self) -> u64 {
        self.0.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED: &str = include_str!("../input/day01_provided.txt");

    #[test]
    fn part1_provided() {
        assert_eq!(part1(PROVIDED).unwrap(), 24000);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 68923);
    }
}
