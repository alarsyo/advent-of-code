use std::fmt::Write;

use anyhow::{Context, Result};
use rand::Rng;

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

    let median_rank = horizontal_positions.len() / 2;
    let median = selection(&mut horizontal_positions, median_rank);

    Ok(horizontal_positions
        .iter()
        // TODO: use abs_diff when stabilized
        .map(|n| abs_diff(*n, median))
        .sum())
}

fn selection<T>(data: &mut [T], i: usize) -> T
where
    T: Copy + Ord,
{
    if data.len() == 1 {
        return data[0];
    }

    let mid = random_partition(data);

    if i < mid {
        selection(&mut data[..mid], i)
    } else {
        selection(&mut data[mid..], i - mid)
    }
}

fn random_partition<T>(data: &mut [T]) -> usize
where
    T: Copy + Ord,
{
    let pivot_index = rand::thread_rng().gen_range(0..data.len());
    let pivot = data[pivot_index];

    let mut i = 0;
    let mut j = data.len() - 1;

    loop {
        while data[i] < pivot {
            i += 1;
        }

        while data[j] > pivot {
            j -= 1;
        }

        if i >= j {
            return usize::max(i, 1);
        }

        data.swap(i, j);
        i += 1;
        j -= 1;
    }
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

    #[test]
    fn test_selection() {
        for _ in 0..4200 {
            for i in 0..=9 {
                let mut data = vec![9, 2, 7, 3, 5, 4, 6, 1, 8, 0];
                let res = selection(&mut data, i);
                assert_eq!(res, i);
            }
        }
    }
}
