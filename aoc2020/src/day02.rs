use std::fmt::Write;
use std::ops::RangeInclusive;
use std::str::FromStr;

use aoc::{err, Result};

const INPUT: &str = include_str!("../input/day02.txt");

#[derive(Debug)]
struct PassPolicy {
    repetitions: RangeInclusive<usize>,
    letter: u8,
    password: String,
}

impl PassPolicy {
    fn new(repetitions: RangeInclusive<usize>, letter: u8, password: String) -> Self {
        Self {
            repetitions,
            letter,
            password,
        }
    }

    #[allow(clippy::naive_bytecount)] // let's not pull a crate just for that
    fn is_valid(&self) -> bool {
        let occurrences = self
            .password
            .as_bytes()
            .iter()
            .filter(|&&b| b == self.letter)
            .count();

        self.repetitions.contains(&occurrences)
    }
}

impl FromStr for PassPolicy {
    type Err = aoc::Error;

    fn from_str(s: &str) -> Result<Self> {
        let s = s.trim_end();

        let space = s
            .find(' ')
            .ok_or_else(|| err!("couldn't parse password policy: didn't find space"))?;
        let dash = s
            .find('-')
            .ok_or_else(|| err!("couldn't parse password policy: didn't find dash"))?;

        let min_bound = s[..dash]
            .parse::<usize>()
            .map_err(|e| err!("couldn't  parse range: {}", e))?;
        let max_bound = s[(dash + 1)..space]
            .parse::<usize>()
            .map_err(|e| err!("couldn't parse range: {}", e))?;

        let colon = s
            .find(':')
            .ok_or_else(|| err!("couldn't parse password policy: didn't find colon"))?;

        let letter = s.as_bytes()[colon - 1];

        let password = &s[(colon + 2)..];

        Ok(Self::new(
            min_bound..=max_bound,
            letter,
            password.to_string(),
        ))
    }
}

#[derive(Debug)]
struct PassPolicyNew {
    pos1: usize,
    pos2: usize,
    letter: u8,
    password: String,
}

impl PassPolicyNew {
    fn new(pos1: usize, pos2: usize, letter: u8, password: String) -> Self {
        Self {
            pos1,
            pos2,
            letter,
            password,
        }
    }

    fn is_valid(&self) -> bool {
        let bytes = self.password.as_bytes();
        (bytes[self.pos1 - 1] == self.letter) ^ (bytes[self.pos2 - 1] == self.letter)
    }
}

impl FromStr for PassPolicyNew {
    type Err = aoc::Error;

    fn from_str(s: &str) -> Result<Self> {
        let s = s.trim_end();

        let space = s
            .find(' ')
            .ok_or_else(|| err!("couldn't parse password policy: didn't find space"))?;
        let dash = s
            .find('-')
            .ok_or_else(|| err!("couldn't parse password policy: didn't find dash"))?;

        let pos1 = s[..dash]
            .parse::<usize>()
            .map_err(|e| err!("couldn't  parse position: {}", e))?;
        let pos2 = s[(dash + 1)..space]
            .parse::<usize>()
            .map_err(|e| err!("couldn't parse position: {}", e))?;

        let colon = s
            .find(':')
            .ok_or_else(|| err!("couldn't parse password policy: didn't find colon"))?;

        let letter = s.as_bytes()[colon - 1];

        let password = &s[(colon + 2)..];

        Ok(Self::new(pos1, pos2, letter, password.to_string()))
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
        .map(|line| line.parse::<PassPolicy>().map_err(|e| err!("{}", e)))
        .collect::<Result<Vec<PassPolicy>>>()?;

    Ok(policies.iter().filter(|policy| policy.is_valid()).count())
}

fn part2(input: &str) -> Result<usize> {
    let policies = input
        .lines()
        .map(|line| line.parse::<PassPolicyNew>().map_err(|e| err!("{}", e)))
        .collect::<Result<Vec<PassPolicyNew>>>()?;

    Ok(policies.iter().filter(|policy| policy.is_valid()).count())
}

#[cfg(test)]
mod tests {
    use super::*;

    static PROVIDED1: &'static str = "1-3 a: abcde";
    static PROVIDED2: &'static str = "1-3 b: cdefg";
    static PROVIDED3: &'static str = "2-9 c: ccccccccc";

    #[test]
    fn part1_provided() {
        assert!(PROVIDED1.parse::<PassPolicy>().unwrap().is_valid());
        assert!(!PROVIDED2.parse::<PassPolicy>().unwrap().is_valid());
        assert!(PROVIDED3.parse::<PassPolicy>().unwrap().is_valid());
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 556);
    }

    #[test]
    fn part2_provided() {
        assert!(PROVIDED1.parse::<PassPolicyNew>().unwrap().is_valid());
        assert!(!PROVIDED2.parse::<PassPolicyNew>().unwrap().is_valid());
        assert!(!PROVIDED3.parse::<PassPolicyNew>().unwrap().is_valid());
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT).unwrap(), 605);
    }
}
