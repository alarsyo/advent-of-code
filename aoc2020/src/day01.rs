use std::fmt::Write;

use aoc::{err, Result};

const INPUT: &str = include_str!("../input/day01.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;

    Ok(res)
}

fn part1(input: &str) -> Result<u64> {
    let entries = input
        .lines()
        .map(|line| line.parse::<u64>().map_err(|e| err!("{}", e)))
        .collect::<Result<Vec<u64>>>()?;

    let (a, b) = find_2020_sum(&entries)?;

    Ok(a * b)
}

fn find_2020_sum(entries: &[u64]) -> Result<(u64, u64)> {
    for i in 0..(entries.len()) {
        for j in 0..entries.len() {
            if i == j {
                continue;
            }
            if entries[i] + entries[j] == 2020 {
                return Ok((entries[i], entries[j]));
            }
        }
    }

    Err(err!("couldn't find 2 elements that sum to 2020"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_provided() {
        let (a, b) = find_2020_sum(&vec![1721, 979, 366, 299, 675, 1456]).unwrap();
        assert_eq!(a, 1721);
        assert_eq!(b, 299);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 1014171);
    }
}
