use std::fmt::Write;

use anyhow::{Context, Result};

const INPUT: &str = include_str!("../input/day07.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;

    Ok(res)
}

fn part1(input: &str) -> Result<u64> {
    let mut horizontal_positions = input
        .trim()
        .split(',')
        .map(|n| n.parse::<u64>().context("couldn't parse position"))
        .collect::<Result<Vec<_>>>()?;

    // TODO: try linear selection algorithm
    horizontal_positions.sort_unstable();
    let median = horizontal_positions[horizontal_positions.len() / 2];

    Ok(horizontal_positions
        .iter()
        // TODO: use abs_diff when stabilized
        .map(|n| n.max(&median) - n.min(&median))
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED: &str = include_str!("../input/day07_provided.txt");

    #[test]
    fn part1_provided() {
        assert_eq!(part1(PROVIDED).unwrap(), 37);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 340056);
    }
}
