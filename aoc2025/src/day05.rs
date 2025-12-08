use std::{
    fmt::Write,
    ops::{Bound, RangeBounds},
};

use anyhow::{Context, Result};

const INPUT: &str = include_str!("../input/day05.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);
    writeln!(res, "part 1: {}", part1(INPUT)?)?;
    writeln!(res, "part 2: {}", part2(INPUT)?)?;
    Ok(res)
}

fn part1(input: &str) -> Result<u64> {
    let mut fresh_id_ranges = Vec::new();
    let mut available_ids = Vec::new();

    let mut lines = input.lines();
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        let (left, right) = line
            .split_once('-')
            .with_context(|| format!("failed to split range `{}' on `-'", line))?;
        let (left, right) = (left.parse::<u64>()?, right.parse::<u64>()?);
        let range = left..=right;
        fresh_id_ranges.push(range);
    }

    for line in lines {
        available_ids.push(line.parse::<u64>()?);
    }

    let mut count = 0;
    for id in available_ids {
        for range in &fresh_id_ranges {
            if range.contains(&id) {
                count += 1;
                break;
            }
        }
    }

    Ok(count)
}

fn part2(input: &str) -> Result<usize> {
    let mut fresh_id_ranges = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            break;
        }
        let (left, right) = line
            .split_once('-')
            .with_context(|| format!("failed to split range `{}' on `-'", line))?;
        let (left, right) = (left.parse::<u64>()?, right.parse::<u64>()?);
        let range = left..=right;
        fresh_id_ranges.push(range);
    }

    // sort all ranges to prepare for merging of overlapping ranges
    let mut new_ranges = Vec::new();
    fresh_id_ranges.sort_by_key(|range| match (range.start_bound(), range.end_bound()) {
        (Bound::Included(&beg), Bound::Included(&end)) => (beg, end),
        _ => unreachable!("We're only handling inclusive ranges"),
    });

    let mut fresh_id_ranges = fresh_id_ranges.into_iter();
    new_ranges.push(
        fresh_id_ranges
            .next()
            .context("Input did not contain any ranges!")?,
    );
    for range in fresh_id_ranges {
        let latest = new_ranges
            .last_mut()
            .expect("new_ranges always contains at least one element");
        let (last_start, last_end) = match (latest.start_bound(), latest.end_bound()) {
            (Bound::Included(&beg), Bound::Included(&end)) => (beg, end),
            _ => unreachable!("We're only handling inclusive ranges"),
        };
        let (cur_start, cur_end) = match (range.start_bound(), range.end_bound()) {
            (Bound::Included(&beg), Bound::Included(&end)) => (beg, end),
            _ => unreachable!("We're only handling inclusive ranges"),
        };

        if cur_start <= last_end + 1 {
            *latest = (last_start)..=(last_end.max(cur_end));
        } else {
            new_ranges.push(range);
        }
    }

    Ok(new_ranges.into_iter().map(|r| r.count()).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED: &str = include_str!("../input/day05_provided.txt");

    #[test]
    fn part1_provided() {
        assert_eq!(part1(PROVIDED).unwrap(), 3);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 509);
    }

    #[test]
    fn part2_provided() {
        assert_eq!(part2(PROVIDED).unwrap(), 14);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT).unwrap(), 336790092076620);
    }
}
