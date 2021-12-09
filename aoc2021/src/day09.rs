use std::fmt::Write;

use anyhow::{Context, Result};

const INPUT: &str = include_str!("../input/day09.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;

    Ok(res)
}

fn part1(input: &str) -> Result<u64> {
    let height_map: HeightMap = input.parse()?;

    Ok(height_map
        .low_points()
        .map(|(x, y)| height_map.risk_level(x, y))
        .sum())
}

#[derive(Clone, Copy)]
enum Neighbour {
    Up,
    Down,
    Left,
    Right,
}

impl Neighbour {
    fn apply(&self, x: usize, y: usize, width: usize, height: usize) -> Option<(usize, usize)> {
        match self {
            Neighbour::Left if x > 0 => Some((x - 1, y)),
            Neighbour::Right if x < width - 1 => Some((x + 1, y)),
            Neighbour::Up if y > 0 => Some((x, y - 1)),
            Neighbour::Down if y < height - 1 => Some((x, y + 1)),
            _ => None,
        }
    }

    const ALL: &'static [Self] = &[
        Neighbour::Left,
        Neighbour::Right,
        Neighbour::Up,
        Neighbour::Down,
    ];
}

#[derive(Debug)]
struct HeightMap {
    heights: Vec<u8>,
    width: usize,
    height: usize,
}

impl HeightMap {
    fn low_points(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..self.height)
            .flat_map(|y| (0..self.width).map(move |x| (x, y)))
            .filter(|&(x, y)| {
                self.neighbours(x, y)
                    .all(|(nx, ny)| self.get(x, y) < self.get(nx, ny))
            })
    }

    fn neighbours(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> + '_ {
        Neighbour::ALL
            .iter()
            .copied()
            .filter_map(move |neighbour| neighbour.apply(x, y, self.width, self.height))
    }

    fn risk_level(&self, x: usize, y: usize) -> u64 {
        self.get(x, y) as u64 + 1
    }

    fn get(&self, x: usize, y: usize) -> u8 {
        self.heights[y * self.width + x]
    }
}

impl std::str::FromStr for HeightMap {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut heights = Vec::new();

        let mut height = 0;
        let mut width = None;
        for line in s.lines().map(str::trim) {
            let line = line
                .chars()
                .map(|chr| {
                    chr.to_digit(10)
                        .map(|digit| digit as u8)
                        .with_context(|| format!("cannot parse char {} to digit", chr))
                })
                .collect::<Result<Vec<_>>>()?;

            if width.is_none() {
                width = Some(line.len());
            }

            height += 1;
            heights.extend_from_slice(&line);
        }

        Ok(HeightMap {
            heights,
            width: width.context("0 lines parsed, width never computed")?,
            height,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED: &str = include_str!("../input/day09_provided.txt");

    #[test]
    fn part1_provided() {
        assert_eq!(part1(PROVIDED).unwrap(), 15);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 522);
    }
}
