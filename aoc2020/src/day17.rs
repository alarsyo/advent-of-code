use std::collections::HashSet;
use std::fmt::Write;

use anyhow::{anyhow, Result};

use itertools::iproduct;

const INPUT: &str = include_str!("../input/day17.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;

    Ok(res)
}

fn part1(input: &str) -> Result<usize> {
    let mut pocket_dim: PocketDimension = input.parse()?;

    for _ in 0..6 {
        pocket_dim.update();
    }

    Ok(pocket_dim.points.len())
}

type Point = (i64, i64, i64);

struct PocketDimension {
    points: HashSet<Point>,
}

impl PocketDimension {
    fn neighbours(point: Point) -> impl Iterator<Item = Point> {
        iproduct!(-1..=1, -1..=1, -1..=1)
            .filter(|(x, y, z)| *x != 0 || *y != 0 || *z != 0)
            .map(move |(dx, dy, dz)| (point.0 + dx, point.1 + dy, point.2 + dz))
    }

    fn active_neighbours(&self, point: Point) -> usize {
        Self::neighbours(point)
            .filter(|p| self.points.contains(p))
            .count()
    }

    fn update(&mut self) {
        let mut processed: HashSet<Point> = HashSet::new();
        let mut new = self.points.clone();

        for point in &self.points {
            for neighbour in Self::neighbours(*point).chain(std::iter::once(*point)) {
                if processed.contains(&neighbour) {
                    continue;
                }

                processed.insert(neighbour);

                // if point is active
                if self.points.contains(&neighbour) {
                    if !(2..=3).contains(&self.active_neighbours(neighbour)) {
                        // now inactive
                        new.remove(&neighbour);
                    }
                } else if self.active_neighbours(neighbour) == 3 {
                    new.insert(neighbour);
                }
            }
        }

        self.points = new;
    }
}

impl std::str::FromStr for PocketDimension {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let points = s
            .lines()
            .enumerate()
            .flat_map(|(i, line)| {
                line.chars().enumerate().filter_map(move |(j, c)| match c {
                    '#' => Some(Ok((i as i64, j as i64, 0))),
                    '.' => None,
                    _ => Some(Err(anyhow!("unexpected char: `{}`", c))),
                })
            })
            .collect::<Result<_>>()?;

        Ok(Self { points })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED: &str = include_str!("../input/day17_provided.txt");

    #[test]
    fn part1_provided() {
        assert_eq!(part1(PROVIDED).unwrap(), 112);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 336);
    }
}
