use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fmt::Write;

use anyhow::{Context, Result};

const INPUT: &str = include_str!("../input/day15.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;
    writeln!(res, "part 2: {}", part2(INPUT)?)?;

    Ok(res)
}

fn part1(input: &str) -> Result<u64> {
    let cavern: CavernMap = input.parse()?;

    let lowest_risk = cavern.lowest_risk_path();

    Ok(lowest_risk)
}

fn part2(input: &str) -> Result<u64> {
    let cavern: CavernMap = input.parse()?;
    let cavern = cavern.bigger();

    let lowest_risk = cavern.lowest_risk_path();

    Ok(lowest_risk)
}

#[derive(Debug)]
struct CavernMap {
    height: usize,
    width: usize,
    risk: Vec<u64>,
}

impl CavernMap {
    // typical Dijkstra implementation, using a binary heap as a priority queue
    fn lowest_risk_path(&self) -> u64 {
        let mut visited = Vec::new();
        visited.resize(self.height * self.width, false);
        let mut total_risk = Vec::new();
        total_risk.resize(self.height * self.width, u64::MAX);
        total_risk[self.index(0, 0)] = 0;

        let mut queue = BinaryHeap::new();
        queue.push(Reverse((0, (0, 0))));

        while !queue.is_empty() {
            let Reverse((curr_risk, (x, y))) = queue.pop().unwrap();
            debug_assert_eq!(curr_risk, total_risk[self.index(x, y)]);

            if (x, y) == (self.width - 1, self.height - 1) {
                // reached destination, we're done!
                break;
            }

            if visited[self.index(x, y)] {
                // duplicate entry in queue, discard
                continue;
            }
            visited[self.index(x, y)] = true;

            for (nx, ny) in self.neighbours(x, y) {
                let neighbour_index = self.index(nx, ny);
                let old_risk = total_risk[neighbour_index];
                let new_risk = curr_risk + self.risk[neighbour_index];

                if new_risk < old_risk {
                    total_risk[neighbour_index] = new_risk;
                    // we don't delete older queue entries for the same cell, if we find them later
                    // on we can just skip them because they're marked as visited already
                    queue.push(Reverse((new_risk, (nx, ny))));
                }
            }
        }

        *total_risk.last().unwrap()
    }

    fn neighbours(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> + 'static {
        let width = self.width;
        let height = self.height;
        Neighbour::ALL
            .iter()
            .copied()
            .filter_map(move |neighbour| neighbour.apply(x, y, width, height))
    }

    fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn bigger(self) -> Self {
        let width = self.width * 5;
        let height = self.height * 5;
        let mut risk = self.risk.clone();
        risk.resize(width * height, 0);

        for shift_y in 0..5 {
            for shift_x in 0..5 {
                for y in 0..self.height {
                    for x in 0..self.width {
                        let new_y = shift_y * self.height + y;
                        let new_x = shift_x * self.width + x;
                        let new_index = new_y * width + new_x;

                        let old_risk = self.risk[self.index(x, y)];
                        let mut new_risk = old_risk + shift_x as u64 + shift_y as u64;
                        if new_risk > 9 {
                            new_risk -= 9;
                        }
                        risk[new_index] = new_risk;
                    }
                }
            }
        }

        Self {
            height,
            width,
            risk,
        }
    }
}

impl std::str::FromStr for CavernMap {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut risk = Vec::new();

        let mut height = 0;
        let mut width = None;
        for line in s.lines().map(str::trim) {
            let line = line
                .chars()
                .map(|chr| {
                    chr.to_digit(10)
                        .map(|digit| digit as u64)
                        .with_context(|| format!("cannot parse char {} to digit", chr))
                })
                .collect::<Result<Vec<_>>>()?;

            if width.is_none() {
                width = Some(line.len());
            }

            height += 1;
            risk.extend_from_slice(&line);
        }

        Ok(CavernMap {
            height,
            width: width.context("0 lines parsed, width never computed")?,
            risk,
        })
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED: &str = include_str!("../input/day15_provided.txt");

    #[test]
    fn part1_provided() {
        assert_eq!(part1(PROVIDED).unwrap(), 40);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 562);
    }

    #[test]
    fn part2_provided() {
        assert_eq!(part2(PROVIDED).unwrap(), 315);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT).unwrap(), 2874);
    }
}
