use std::fmt::Write;

use anyhow::{Context, Result};

const INPUT: &str = include_str!("../input/day07.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;
    writeln!(res, "part 2: {}", part2(INPUT)?)?;

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
        .map(|n| abs_diff(*n, median))
        .sum())
}

fn part2(input: &str) -> Result<u64> {
    let horizontal_positions = input
        .trim()
        .split(',')
        .map(|n| n.parse::<u64>().context("couldn't parse position"))
        .collect::<Result<Vec<_>>>()?;

    let min = *horizontal_positions.iter().min().unwrap();
    let max = *horizontal_positions.iter().max().unwrap();

    let minimum_fuel = (min..=max)
        .map(|pos| {
            horizontal_positions
                .iter()
                .map(|n| part2_distance(*n, pos))
                .sum::<f64>()
                .floor() as u64
        })
        .min()
        .unwrap();

    Ok(minimum_fuel)
}

fn abs_diff(a: u64, b: u64) -> u64 {
    a.max(b) - a.min(b)
}

fn part2_distance(a: u64, b: u64) -> f64 {
    let distance = abs_diff(a, b) as f64;

    distance * (distance + 1.0) / 2.0
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

    #[test]
    fn part2_provided() {
        assert_eq!(part2(PROVIDED).unwrap(), 168);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT).unwrap(), 96592275);
    }
}
