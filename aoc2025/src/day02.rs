use anyhow::{Context, Result, anyhow};
use std::{fmt::Write, ops::RangeInclusive, str::FromStr};

const INPUT: &str = include_str!("../input/day02.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);
    writeln!(res, "part 1: {}", part1(INPUT)?)?;
    writeln!(res, "part 2: {}", part2(INPUT)?)?;
    Ok(res)
}

struct IdRange(RangeInclusive<u64>);

impl FromStr for IdRange {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (left, right) = s
            .split_once('-')
            .ok_or_else(|| anyhow!("couldn't find dash in range: `{}'", s))?;
        let (left, right) = (
            left.parse()
                .with_context(|| format!("couldn't parse left member of range: `{}'", left))?,
            right
                .trim()
                .parse()
                .with_context(|| format!("couldn't parse right member of range: `{}'", right))?,
        );

        Ok(IdRange(left..=right))
    }
}

impl Iterator for IdRange {
    type Item = <RangeInclusive<u64> as Iterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

fn get_num_digits(num: &u64) -> usize {
    num.ilog10() as usize + 1
}

fn is_repeated_twice(num: &u64) -> bool {
    let num_digits = get_num_digits(num);
    if !num_digits.is_multiple_of(2) {
        return false;
    }
    let half = num_digits / 2;
    let mask = 10_u64.pow(half as u32);

    *num / mask == *num % mask
}

fn part1(input: &str) -> Result<u64> {
    let ranges = input
        .split(',')
        .map(IdRange::from_str)
        .collect::<Result<Vec<_>>>()?;

    Ok(ranges
        .into_iter()
        .map(|range| range.filter(is_repeated_twice).sum::<u64>())
        .sum())
}

fn has_identical_chunks(num: &u64, chunk_size: usize) -> bool {
    // build a "mask" for our chunk size
    let mask = 10_u64.pow(chunk_size as u32);

    // this is what all chunks should look like to satisfy our condition
    let target = *num % mask;

    let mut num = *num;
    while num > target {
        // shift num right by 10^(sub_digit_num)
        num /= mask;
        if num % mask != target {
            return false;
        }
    }

    true
}

fn is_repeated_at_least_twice(num: &u64) -> bool {
    let num_digits = get_num_digits(num);
    if num_digits < 2 {
        return false;
    }

    has_identical_chunks(num, 1)
        || (2..=(num_digits / 2)).any(|n| {
            // walk all possible divisors of our number of digits
            if !num_digits.is_multiple_of(n) {
                return false;
            }
            let chunk_size = num_digits / n;
            has_identical_chunks(num, chunk_size)
        })
}

fn part2(input: &str) -> Result<u64> {
    let ranges = input
        .split(',')
        .map(IdRange::from_str)
        .collect::<Result<Vec<_>>>()?;

    Ok(ranges
        .into_iter()
        .map(|range| range.filter(is_repeated_at_least_twice).sum::<u64>())
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED: &str = include_str!("../input/day02_provided.txt");

    #[test]
    fn repeated_twice() {
        assert!(is_repeated_twice(&11));
        assert!(is_repeated_twice(&22));
        assert!(is_repeated_twice(&99));
        assert!(is_repeated_twice(&1010));
        assert!(is_repeated_twice(&1188511885));
        assert!(is_repeated_twice(&222222));
        assert!(is_repeated_twice(&446446));
        assert!(is_repeated_twice(&38593859));

        assert!(!is_repeated_twice(&111));
        assert!(!is_repeated_twice(&121));
        assert!(!is_repeated_twice(&1));
    }

    #[test]
    fn part1_provided() {
        assert_eq!(part1(PROVIDED).unwrap(), 1227775554);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 26255179562);
    }

    #[test]
    fn repeated_at_least_twice() {
        assert!(is_repeated_at_least_twice(&11));
        assert!(is_repeated_at_least_twice(&22));
        assert!(is_repeated_at_least_twice(&99));
        assert!(is_repeated_at_least_twice(&111));
        assert!(is_repeated_at_least_twice(&999));
        assert!(is_repeated_at_least_twice(&1010));
        assert!(is_repeated_at_least_twice(&1188511885));
        assert!(is_repeated_at_least_twice(&222222));
        assert!(is_repeated_at_least_twice(&446446));
        assert!(is_repeated_at_least_twice(&38593859));
        assert!(is_repeated_at_least_twice(&565656));
        assert!(is_repeated_at_least_twice(&824824824));
        assert!(is_repeated_at_least_twice(&2121212121));

        assert!(!is_repeated_at_least_twice(&1121));
        assert!(!is_repeated_at_least_twice(&121));
        assert!(!is_repeated_at_least_twice(&1));
    }

    #[test]
    fn part2_provided() {
        assert_eq!(part2(PROVIDED).unwrap(), 4174379265);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT).unwrap(), 31680313976);
    }
}
