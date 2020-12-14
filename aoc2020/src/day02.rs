use std::fmt::Write;
use std::str::FromStr;

use anyhow::{Context, Result};

const INPUT: &str = include_str!("../input/day02.txt");

#[derive(Debug)]
struct PassPolicy {
    min_bound: usize,
    max_bound: usize,
    letter: u8,
    password: String,
}

impl PassPolicy {
    fn new(min_bound: usize, max_bound: usize, letter: u8, password: String) -> Self {
        Self {
            min_bound,
            max_bound,
            letter,
            password,
        }
    }

    #[allow(clippy::naive_bytecount)] // let's not pull a crate just for that
    fn is_valid_part1(&self) -> bool {
        let occurrences = self
            .password
            .as_bytes()
            .iter()
            .filter(|&&b| b == self.letter)
            .count();

        let range = self.min_bound..=self.max_bound;

        range.contains(&occurrences)
    }

    fn is_valid_part2(&self) -> bool {
        let bytes = self.password.as_bytes();
        (bytes[self.min_bound - 1] == self.letter) ^ (bytes[self.max_bound - 1] == self.letter)
    }
}

impl FromStr for PassPolicy {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let s = s.trim_end();

        let space = s
            .find(' ')
            .context("couldn't parse password policy: didn't find space")?;
        let dash = s
            .find('-')
            .context("couldn't parse password policy: didn't find dash")?;

        let min_bound = s[..dash]
            .parse::<usize>()
            .context("couldn't  parse range")?;
        let max_bound = s[(dash + 1)..space]
            .parse::<usize>()
            .context("couldn't parse range")?;

        let colon = s
            .find(':')
            .context("couldn't parse password policy: didn't find colon")?;

        let letter = s.as_bytes()[colon - 1];

        let password = &s[(colon + 2)..];

        Ok(Self::new(
            min_bound,
            max_bound,
            letter,
            password.to_string(),
        ))
    }
}

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;
    writeln!(res, "part 2: {}", part2(INPUT)?)?;

    Ok(res)
}

fn part1(input: &str) -> Result<usize> {
    let policies = input
        .lines()
        .map(|line| line.parse::<PassPolicy>())
        .collect::<Result<Vec<PassPolicy>>>()?;

    Ok(policies
        .iter()
        .filter(|policy| policy.is_valid_part1())
        .count())
}

fn part2(input: &str) -> Result<usize> {
    let policies = input
        .lines()
        .map(|line| line.parse::<PassPolicy>())
        .collect::<Result<Vec<PassPolicy>>>()?;

    Ok(policies
        .iter()
        .filter(|policy| policy.is_valid_part2())
        .count())
}

#[cfg(test)]
mod tests {
    use super::*;

    static PROVIDED1: &'static str = "1-3 a: abcde";
    static PROVIDED2: &'static str = "1-3 b: cdefg";
    static PROVIDED3: &'static str = "2-9 c: ccccccccc";

    #[test]
    fn part1_provided() {
        assert!(PROVIDED1.parse::<PassPolicy>().unwrap().is_valid_part1());
        assert!(!PROVIDED2.parse::<PassPolicy>().unwrap().is_valid_part1());
        assert!(PROVIDED3.parse::<PassPolicy>().unwrap().is_valid_part1());
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 556);
    }

    #[test]
    fn part2_provided() {
        assert!(PROVIDED1.parse::<PassPolicy>().unwrap().is_valid_part2());
        assert!(!PROVIDED2.parse::<PassPolicy>().unwrap().is_valid_part2());
        assert!(!PROVIDED3.parse::<PassPolicy>().unwrap().is_valid_part2());
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT).unwrap(), 605);
    }
}
