use std::fmt::Write;

use anyhow::Result;

const INPUT: &str = include_str!("../input/day01.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;
    writeln!(res, "part 2: {}", part2(INPUT)?)?;

    Ok(res)
}

fn fuel_needed(module_weight: u64) -> u64 {
    (module_weight / 3).saturating_sub(2)
}

fn part1(input: &str) -> Result<u64> {
    input
        .lines()
        .map(str::parse)
        .map(|w| w.map(fuel_needed).map_err(anyhow::Error::new))
        .sum()
}

fn cumulated_fuel_needed(module_weight: u64) -> u64 {
    let mut total_fuel = fuel_needed(module_weight);
    let mut additional_fuel = fuel_needed(total_fuel);

    while additional_fuel != 0 {
        total_fuel += additional_fuel;
        additional_fuel = fuel_needed(additional_fuel);
    }

    total_fuel
}

fn part2(input: &str) -> Result<u64> {
    input
        .lines()
        .map(str::parse)
        .map(|w| w.map(cumulated_fuel_needed).map_err(anyhow::Error::new))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_provided() {
        assert_eq!(fuel_needed(12), 2);
        assert_eq!(fuel_needed(14), 2);
        assert_eq!(fuel_needed(1969), 654);
        assert_eq!(fuel_needed(100756), 33583);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 3268951);
    }

    #[test]
    fn part2_provided() {
        assert_eq!(cumulated_fuel_needed(14), 2);
        assert_eq!(cumulated_fuel_needed(1969), 966);
        assert_eq!(cumulated_fuel_needed(100756), 50346);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT).unwrap(), 4900568);
    }
}
