use std::collections::HashSet;
use std::fmt::Write;

use anyhow::{Context, Result};

const INPUT: &str = include_str!("../input/day09.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;
    writeln!(res, "part 2: {}", part2(INPUT)?)?;

    Ok(res)
}

fn part1(input: &str) -> Result<u64> {
    let height_map: HeightMap = input.parse()?;

    Ok(height_map
        .low_points()
        .map(|(x, y)| height_map.risk_level(x, y))
        .sum())
}

fn part2(input: &str) -> Result<u64> {
    let mut height_map: HeightMap = input.parse()?;

    let low_points: Vec<_> = height_map.low_points().collect();

    let mut bassin_sizes: Vec<_> = low_points
        .iter()
        .map(|&(x, y)| height_map.fill_basin(x, y))
        .collect();
    bassin_sizes.sort_unstable();

    bassin_sizes
        .iter()
        .copied()
        .skip(bassin_sizes.len() - 3)
        .reduce(|acc, elem| acc * elem)
        .context("couldn't find 3 bassins")
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
    filled_points: HashSet<(usize, usize)>,
}

impl HeightMap {
    fn fill_basin(&mut self, x: usize, y: usize) -> u64 {
        if self.get(x, y) == 9 {
            return 0;
        }

        if self.filled_points.contains(&(x, y)) {
            return 0;
        }
        self.filled_points.insert((x, y));

        let neighbours: Vec<_> = self.neighbours(x, y).collect();
        neighbours
            .iter()
            .map(|&(nx, ny)| self.fill_basin(nx, ny))
            .sum::<u64>()
            + 1
    }

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
            filled_points: HashSet::new(),
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

    #[test]
    fn part2_provided() {
        assert_eq!(part2(PROVIDED).unwrap(), 1134);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT).unwrap(), 916688);
    }
}
