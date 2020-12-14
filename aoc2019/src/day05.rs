use std::fmt::Write;

use anyhow::{Context, Result};

use crate::intcode::Intcode;

const INPUT: &str = include_str!("../input/day05.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;
    writeln!(res, "part 2: {}", part2(INPUT)?)?;

    Ok(res)
}

fn part1(input: &str) -> Result<i64> {
    let mut intcode = Intcode::new(input)?;
    intcode.add_input(1);
    intcode.run()?;
    intcode.get_last_output().context("intcode gave no output")
}

fn part2(input: &str) -> Result<i64> {
    let mut intcode = Intcode::new(input)?;
    intcode.add_input(5);
    intcode.run()?;
    intcode.get_last_output().context("intcode gave no output")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 16225258);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT).unwrap(), 2808771);
    }
}
