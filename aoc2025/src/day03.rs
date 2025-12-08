use std::{cmp, fmt::Write};

use anyhow::{Context, Result};

const INPUT: &str = include_str!("../input/day03.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);
    writeln!(res, "part 1: {}", part1(INPUT)?)?;
    Ok(res)
}

fn find_max_tens_digit(bank: &[u8]) -> Option<(usize, u8)> {
    // We don't include the last byte since it cannot be the digit for tens, otherwise we'd have no
    // byte left for the units digit.
    bank[..bank.len() - 1]
        .iter()
        .copied()
        .enumerate()
        // We want the max, but use min + Reverse because min selects the FIRST minimum, while max
        // selects the LAST maximum, and we want the first (to leave more choices available to us
        // for the units digit).
        .min_by_key(|&(_idx, val)| cmp::Reverse(val))
}

fn find_max_units_digit(sub_bank: &[u8]) -> Option<u8> {
    sub_bank.iter().copied().max()
}

fn compute_max_joltage(bank: &[u8]) -> Result<u64> {
    let (idx, tens) = find_max_tens_digit(bank).context("couldn't find tens digit in bank")?;
    let units =
        find_max_units_digit(&bank[(idx + 1)..]).context("couldn't find units digit in bank")?;
    let (tens, units) = ((tens - b'0') as u64, (units - b'0') as u64);
    Ok(tens * 10 + units)
}

fn part1(input: &str) -> Result<u64> {
    let mut sum = 0;
    for line in input.lines() {
        let joltage = compute_max_joltage(line.as_bytes())
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
            compute_max_joltage("987654321111111".as_bytes()).ok(),
            Some(98)
        );
        assert_eq!(
            compute_max_joltage("811111111111119".as_bytes()).ok(),
            Some(89)
        );
        assert_eq!(
            compute_max_joltage("234234234234278".as_bytes()).ok(),
            Some(78)
        );
        assert_eq!(
            compute_max_joltage("818181911112111".as_bytes()).ok(),
            Some(92)
        );
        // added this test to check for min + Reverse usage
        assert_eq!(compute_max_joltage("9892".as_bytes()).ok(), Some(99));
        assert_eq!(part1(PROVIDED).unwrap(), 357);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 17034);
    }

    //#[test]
    //fn part2_provided() {
    //    assert_eq!(part2(PROVIDED).unwrap(), 4174379265);
    //}

    //#[test]
    //fn part2_real() {
    //    assert_eq!(part2(INPUT).unwrap(), 31680313976);
    //}
}
