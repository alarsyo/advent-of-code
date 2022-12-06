use std::fmt::Write;

use anyhow::{Context, Result};

const INPUT: &str = include_str!("../input/day03.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;
    writeln!(res, "part 2: {}", part2(INPUT)?)?;

    Ok(res)
}

fn part1(input: &str) -> Result<u64> {
    let binary_numbers: Vec<&str> = input.lines().collect();
    // all binary numbers should have the same length
    let size = binary_numbers[0].len();

    let gamma = compute_gamma(&binary_numbers, size);
    let epsilon = compute_epsilon(gamma, size);

    Ok(gamma * epsilon)
}

/// Each bit in the gamma rate can be determined by finding the most common bit in the corresponding
/// position of all numbers in the diagnostic report.
fn compute_gamma(binary_numbers: &[&str], size: usize) -> u64 {
    let mut gamma = 0;

    for pos in 0..size {
        let digit = u64::from(count_ones(binary_numbers, pos) > (binary_numbers.len() / 2));
        gamma = (gamma << 1) | digit;
    }

    gamma
}

fn count_ones(binary_numbers: &[&str], pos: usize) -> usize {
    binary_numbers
        .iter()
        .filter(|&&num| num.chars().nth(pos).unwrap() == '1')
        .count()
}

/// The epsilon rate is calculated in a similar way; rather than use the most common bit, the least
/// common bit from each position is used.
///
/// We can just use flip every bit in gamma (respecting the size of the input)
fn compute_epsilon(gamma: u64, size: usize) -> u64 {
    // mask 0b000000000000000011111111 with `size` 1s.
    let shift = u64::BITS - (size as u32);
    let mask = (u64::MAX << shift) >> shift;

    (!gamma) & mask
}

fn part2(input: &str) -> Result<u64> {
    let binary_numbers: Vec<&str> = input.lines().collect();

    // all binary numbers should have the same length
    let size = binary_numbers[0].len();
    #[cfg(debug_assertions)]
    binary_numbers.iter().for_each(|num| {
        debug_assert_eq!(num.len(), size);
    });

    let oxygen_generator_rating = compute_oxygen_generator_rating(&binary_numbers, size)?;
    let co2_scrubber_rating = compute_co2_scrubber_rating(&binary_numbers, size)?;

    Ok(oxygen_generator_rating * co2_scrubber_rating)
}

fn filter_by_strat<Strat>(binary_numbers: &[&str], size: usize, strat: Strat) -> Result<u64>
where
    Strat: Fn(bool) -> char,
{
    let mut numbers = binary_numbers.to_vec();

    for pos in 0..size {
        if numbers.len() == 1 {
            // only one number left, we're done!
            break;
        }

        let one_is_more_common = count_ones(&numbers, pos) >= ((numbers.len() + 1) / 2);
        let digit_of_interest = strat(one_is_more_common);

        numbers.retain(|number| number.chars().nth(pos).unwrap() == digit_of_interest);
    }

    debug_assert_eq!(numbers.len(), 1);

    u64::from_str_radix(numbers[0], 2).context("couldn't parse binary number")
}

/// To find oxygen generator rating, determine the most common value (0 or 1) in the current bit
/// position, and keep only numbers with that bit in that position. If 0 and 1 are equally common,
/// keep values with a 1 in the position being considered.
fn compute_oxygen_generator_rating(binary_numbers: &[&str], size: usize) -> Result<u64> {
    filter_by_strat(binary_numbers, size, |one_is_more_common| {
        if one_is_more_common {
            '1'
        } else {
            '0'
        }
    })
}

/// To find CO2 scrubber rating, determine the least common value (0 or 1) in the current bit
/// position, and keep only numbers with that bit in that position. If 0 and 1 are equally
/// common, keep values with a 0 in the position being considered.
fn compute_co2_scrubber_rating(binary_numbers: &[&str], size: usize) -> Result<u64> {
    filter_by_strat(binary_numbers, size, |one_is_more_common| {
        if one_is_more_common {
            '0'
        } else {
            '1'
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED: &str = include_str!("../input/day03_provided.txt");

    #[test]
    fn part1_provided() {
        let binary_numbers: Vec<&str> = PROVIDED.lines().collect();
        let size = binary_numbers[0].len();

        let gamma = compute_gamma(&binary_numbers, size);
        assert_eq!(gamma, 22);

        let epsilon = compute_epsilon(gamma, size);
        assert_eq!(epsilon, 9);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 3429254);
    }

    #[test]
    fn part2_provided() {
        let binary_numbers: Vec<&str> = PROVIDED.lines().collect();
        let size = binary_numbers[0].len();

        let oxygen_generator_rating =
            compute_oxygen_generator_rating(&binary_numbers, size).unwrap();
        assert_eq!(oxygen_generator_rating, 23);

        let co2_scrubber_rating = compute_co2_scrubber_rating(&binary_numbers, size).unwrap();
        assert_eq!(co2_scrubber_rating, 10);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT).unwrap(), 5410338);
    }
}
