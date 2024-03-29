use std::fmt::Write;
use std::str::FromStr;

use anyhow::{Context, Result};

const INPUT: &str = include_str!("../input/day02.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    let presents: Vec<Present> = INPUT.lines().map(str::parse).collect::<Result<_>>()?;

    writeln!(res, "part 1: {}", part1(&presents))?;
    writeln!(res, "part 2: {}", part2(&presents))?;

    Ok(res)
}

fn wrapping_paper(present: &Present) -> u64 {
    let faces_surfaces = &[
        present.length * present.width,
        present.width * present.height,
        present.height * present.length,
    ];

    let total_area: u64 = faces_surfaces.iter().map(|face| face * 2).sum();

    total_area + faces_surfaces.iter().min().unwrap()
}

fn part1(presents: &[Present]) -> u64 {
    presents.iter().map(wrapping_paper).sum()
}

fn ribbon_bow_length(present: &Present) -> u64 {
    present.length * present.width * present.height
}

fn ribbon_needed(present: &Present) -> u64 {
    let faces_perimeters = &[
        2 * present.length + 2 * present.width,
        2 * present.width + 2 * present.height,
        2 * present.height + 2 * present.length,
    ];

    let smallest_perimeter = faces_perimeters.iter().min().unwrap();

    smallest_perimeter + ribbon_bow_length(present)
}

fn part2(presents: &[Present]) -> u64 {
    presents.iter().map(ribbon_needed).sum()
}

struct Present {
    length: u64,
    width: u64,
    height: u64,
}

impl FromStr for Present {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let x = s
            .find('x')
            .with_context(|| format!("couldn't find first `x` in {}", s))?;

        let length = s[..x].parse()?;

        let s = &s[(x + 1)..];
        let x = s
            .find('x')
            .with_context(|| format!("couldn't find second `x` in {}", s))?;

        let width = s[..x].parse()?;

        let s = &s[(x + 1)..].trim_end();

        let height = s.parse()?;

        Ok(Present {
            length,
            width,
            height,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_provided() {
        assert_eq!(wrapping_paper(&"2x3x4".parse().unwrap()), 58);
        assert_eq!(wrapping_paper(&"1x1x10".parse().unwrap()), 43);
    }

    #[test]
    fn part1_real() {
        let presents: Vec<Present> = INPUT
            .lines()
            .map(str::parse)
            .collect::<Result<_>>()
            .unwrap();

        assert_eq!(part1(&presents), 1598415);
    }

    #[test]
    fn part2_provided() {
        assert_eq!(ribbon_needed(&"2x3x4".parse().unwrap()), 34);
        assert_eq!(ribbon_needed(&"1x1x10".parse().unwrap()), 14);
    }

    #[test]
    fn part2_real() {
        let presents: Vec<Present> = INPUT
            .lines()
            .map(str::parse)
            .collect::<Result<_>>()
            .unwrap();

        assert_eq!(part2(&presents), 3812909);
    }
}
