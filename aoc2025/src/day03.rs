use std::{cmp, fmt::Write};

use anyhow::{Context, Result};

const INPUT: &str = include_str!("../input/day03.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);
    writeln!(res, "part 1: {}", part1(INPUT)?)?;
    writeln!(res, "part 2: {}", part2(INPUT)?)?;
    Ok(res)
}

fn find_max_digit(bank: &[u8]) -> Option<(usize, u8)> {
    bank.iter()
        .copied()
        .enumerate()
        // We want the max, but use min + Reverse because min selects the FIRST minimum, while max
        // selects the LAST maximum, and we want the first (to leave more choices available to us
        // for the units digit).
        .min_by_key(|&(_idx, val)| cmp::Reverse(val))
}

fn compute_max_joltage(mut bank: &[u8], batteries: usize) -> Result<u64> {
    let mut joltage = 0;
    for battery in 0..batteries {
        // We don't include the last [batteries - battery - 1] bytes since we need at least these other
        // digits for the final number
        let not_in_search = batteries - battery - 1;
        let (idx, digit) = find_max_digit(&bank[..(bank.len() - not_in_search)])
            .context("couldn't find max digit in bank")?;

        let digit = (digit - b'0') as u64;
        joltage = joltage * 10 + digit;
        bank = &bank[(idx + 1)..];
    }
    Ok(joltage)
}

fn part1(input: &str) -> Result<u64> {
    let mut sum = 0;
    for line in input.lines() {
        let joltage = compute_max_joltage(line.as_bytes(), 2)
            .with_context(|| format!("couldn't compute joltage for bank `{}'", line))?;
        sum += joltage;
    }

    Ok(sum)
}

fn part2(input: &str) -> Result<u64> {
    let mut sum = 0;
    for line in input.lines() {
        let joltage = compute_max_joltage(line.as_bytes(), 12)
            .with_context(|| format!("couldn't compute joltage for bank `{}'", line))?;
        sum += joltage;
    }

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED: &str = include_str!("../input/day03_provided.txt");

    #[test]
    fn part1_provided() {
        assert_eq!(
            compute_max_joltage("987654321111111".as_bytes(), 2).ok(),
            Some(98)
        );
        assert_eq!(
            compute_max_joltage("811111111111119".as_bytes(), 2).ok(),
            Some(89)
        );
        assert_eq!(
            compute_max_joltage("234234234234278".as_bytes(), 2).ok(),
            Some(78)
        );
        assert_eq!(
            compute_max_joltage("818181911112111".as_bytes(), 2).ok(),
            Some(92)
        );
        // added this test to check for min + Reverse usage
        assert_eq!(compute_max_joltage("9892".as_bytes(), 2).ok(), Some(99));
        assert_eq!(part1(PROVIDED).unwrap(), 357);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 17034);
    }

    #[test]
    fn part2_provided() {
        assert_eq!(part2(PROVIDED).unwrap(), 3121910778619);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT).unwrap(), 168798209663590);
    }
}
