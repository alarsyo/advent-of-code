use std::fmt::Write;

use anyhow::Result;

const INPUT: &str = include_str!("../input/day03.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;

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

fn compute_gamma(binary_numbers: &[&str], size: usize) -> u64 {
    let mut gamma = 0;

    for pos in 0..size {
        let digit = if count_ones(binary_numbers, pos) > (binary_numbers.len() / 2) {
            // majority of ones
            1
        } else {
            // majority of zeroes
            0
        };
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

fn compute_epsilon(gamma: u64, size: usize) -> u64 {
    // mask 0b000000000000000011111111 with `size` 1s.
    let shift = u64::BITS - (size as u32);
    let mask = (u64::MAX << shift) >> shift;

    (!gamma) & mask
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
        let epsilon = compute_epsilon(gamma, size);

        assert_eq!(gamma, 22);
        assert_eq!(epsilon, 9);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 3429254);
    }
}
