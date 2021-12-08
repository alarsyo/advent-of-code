use std::fmt::Write;

use anyhow::{Context, Result};

const INPUT: &str = include_str!("../input/day08.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;

    Ok(res)
}

fn part1(input: &str) -> Result<usize> {
    let entries = input
        .lines()
        .map(TryInto::try_into)
        .collect::<Result<Vec<Entry>>>()?;

    Ok(entries.iter().map(Entry::count_easy_digits_in_output).sum())
}

struct Entry<'a> {
    four_digits_output: Vec<&'a str>,
}

impl<'a> Entry<'a> {
    fn count_easy_digits_in_output(&self) -> usize {
        self.four_digits_output
            .iter()
            .filter(|digit| Self::is_easy_digit(digit))
            .count()
    }

    fn is_easy_digit(digit: &str) -> bool {
        matches!(digit.len(), 2 | 3 | 4 | 7)
    }
}

impl<'a> TryFrom<&'a str> for Entry<'a> {
    type Error = anyhow::Error;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let (_, output) = s.split_once(" | ").context("couldn't split on ` | `")?;

        Ok(Self {
            four_digits_output: output.trim().split(' ').collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED: &str = include_str!("../input/day08_provided.txt");

    #[test]
    fn part1_provided() {
        assert_eq!(part1(PROVIDED).unwrap(), 26);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 488);
    }
}
