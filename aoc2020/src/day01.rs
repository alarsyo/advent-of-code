use std::collections::HashSet;
use std::fmt::Write;

use anyhow::{anyhow, Result};

const INPUT: &str = include_str!("../input/day01.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;
    writeln!(res, "part 2: {}", part2(INPUT)?)?;

    Ok(res)
}

fn part1(input: &str) -> Result<i64> {
    let entries = input
        .lines()
        .map(|line| line.parse::<i64>().map_err(anyhow::Error::new))
        .collect::<Result<Vec<i64>>>()?;

    let (a, b) = find_2020_2_sum(&entries)?;

    Ok(a * b)
}

fn part2(input: &str) -> Result<i64> {
    let entries = input
        .lines()
        .map(|line| line.parse::<i64>().map_err(anyhow::Error::new))
        .collect::<Result<Vec<_>>>()?;

    let (a, b, c) = find_2020_3_sum(&entries)?;

    Ok(a * b * c)
}

fn find_2020_2_sum(entries: &[i64]) -> Result<(i64, i64)> {
    let set: HashSet<i64> = entries.iter().copied().collect();

    for a in entries {
        let b = 2020 - a;

        if set.contains(&b) {
            return Ok((*a, b));
        }
    }

    Err(anyhow!("couldn't find 2 elements that sum to 2020"))
}

fn find_2020_3_sum(entries: &[i64]) -> Result<(i64, i64, i64)> {
    let set: HashSet<i64> = entries.iter().copied().collect();

    for i in 0..entries.len() {
        for j in 0..entries.len() {
            if i == j {
                continue;
            }

            let a = entries[i];
            let b = entries[j];

            let c = 2020 - a - b;

            if set.contains(&c) {
                return Ok((a, b, c));
            }
        }
    }

    Err(anyhow!("couldn't find 2 elements that sum to 2020"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED: &[i64] = &[1721, 979, 366, 299, 675, 1456];

    #[test]
    fn part1_provided() {
        let (a, b) = find_2020_2_sum(PROVIDED).unwrap();
        assert_eq!(a + b, 2020);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 1014171);
    }

    #[test]
    fn part2_provided() {
        let (a, b, c) = find_2020_3_sum(PROVIDED).unwrap();
        assert_eq!(a + b + c, 2020);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT).unwrap(), 46584630);
    }
}
