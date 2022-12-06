use std::{collections::BTreeSet, fmt::Write, str::FromStr};

use anyhow::{bail, Context, Result};

const INPUT: &str = include_str!("../input/day03.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;
    writeln!(res, "part 2: {}", part2(INPUT)?)?;

    Ok(res)
}

fn part1(input: &str) -> Result<u64> {
    let mut priorities = 0;
    for line in input.lines() {
        let rucksack = RucksackSplit::from_str(line)?;
        let item = rucksack
            .find_duplicate()
            .context("rucksack didn't have any duplicate!")?;
        priorities += item.priority();
    }

    Ok(priorities)
}

fn part2(input: &str) -> Result<u64> {
    let mut priorities = 0;
    let mut lines = input.lines();
    while let (Some(first), Some(second), Some(third)) = (lines.next(), lines.next(), lines.next())
    {
        let first = Rucksack::from_str(first)?;
        let second = Rucksack::from_str(second)?;
        let third = Rucksack::from_str(third)?;

        let priority = first
            .items
            .intersection(&second.items)
            .copied()
            .collect::<BTreeSet<_>>()
            .intersection(&third.items)
            .next()
            .context("the three elves didn't have any item in common!")?
            .priority();

        priorities += priority;
    }

    Ok(priorities)
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Item(u8);

impl TryFrom<u8> for Item {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'A'..=b'Z' | b'a'..=b'z' => Ok(Self(value)),
            _ => bail!("unsupported item kind: `{}'", value),
        }
    }
}

impl Item {
    fn priority(self) -> u64 {
        match self.0 {
            b'A'..=b'Z' => (self.0 - b'A' + 27).into(),
            b'a'..=b'z' => (self.0 - b'a' + 1).into(),
            _ => unreachable!("shouldn't happen if Item is constructed from TryFrom"),
        }
    }
}

struct RucksackSplit {
    first_half: BTreeSet<Item>,
    second_half: BTreeSet<Item>,
}

impl std::str::FromStr for RucksackSplit {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() % 2 != 0 {
            bail!(
                "rucksack should contain an even number of items, this one contained {}",
                s.len()
            );
        }

        let half = s.len() / 2;
        let mut bytes = s.bytes();
        let first_half = (&mut bytes)
            .take(half)
            .map(Item::try_from)
            .collect::<Result<_>>()?;

        let second_half = (&mut bytes)
            .take(half)
            .map(Item::try_from)
            .collect::<Result<_>>()?;

        debug_assert!(bytes.count() == 0, "something went terribly wrong!");

        Ok(Self {
            first_half,
            second_half,
        })
    }
}

impl RucksackSplit {
    fn find_duplicate(&self) -> Option<Item> {
        self.first_half
            .intersection(&self.second_half)
            .next()
            .copied()
    }
}

struct Rucksack {
    items: BTreeSet<Item>,
}

impl std::str::FromStr for Rucksack {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bytes = s.bytes();
        let items = (&mut bytes).map(Item::try_from).collect::<Result<_>>()?;

        Ok(Self { items })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED: &str = include_str!("../input/day03_provided.txt");

    #[test]
    fn part1_provided() {
        assert_eq!(part1(PROVIDED).unwrap(), 157);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 8018);
    }

    #[test]
    fn part2_provided() {
        assert_eq!(part2(PROVIDED).unwrap(), 70);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT).unwrap(), 2518);
    }
}
