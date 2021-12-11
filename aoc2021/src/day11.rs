use std::fmt::Write;

use anyhow::{Context, Result};

const INPUT: &str = include_str!("../input/day11.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;
    writeln!(res, "part 2: {}", part2(INPUT)?)?;

    Ok(res)
}

fn part1(input: &str) -> Result<usize> {
    let mut grid: OctopusGrid = input.parse()?;

    Ok((0..100).map(|_| grid.step()).sum())
}

fn part2(input: &str) -> Result<usize> {
    let mut grid: OctopusGrid = input.parse()?;

    let (_, step) = (0..)
        .map(|step| (grid.step(), step))
        .find(|(flashes, _)| *flashes == 100)
        .context("never reached step where octopuses all flashed simultaneously")?;

    Ok(step + 1)
}

#[derive(Clone, Copy, Debug)]
struct Octopus {
    energy_level: u8,
    has_flashed: bool,
}

impl Octopus {
    fn increment_energy_level(&mut self) {
        self.energy_level += 1;
    }

    fn reset_energy_level(&mut self) {
        if self.has_flashed {
            self.energy_level = 0;
            self.has_flashed = false;
        }
    }

    fn flashes(&mut self) -> bool {
        if self.energy_level > 9 && !self.has_flashed {
            self.has_flashed = true;
            return true;
        }

        false
    }
}

impl TryFrom<char> for Octopus {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self> {
        Ok(Octopus {
            energy_level: value
                .to_digit(10)
                .with_context(|| format!("couldn't convert char `{}` to digit", value))?
                as u8,
            has_flashed: false,
        })
    }
}

#[derive(Clone, Copy)]
enum Neighbour {
    Up,
    Down,
    Left,
    Right,
    UpperLeft,
    UpperRight,
    LowerLeft,
    LowerRight,
}

impl Neighbour {
    fn apply(&self, x: usize, y: usize, width: usize, height: usize) -> Option<(usize, usize)> {
        match self {
            Neighbour::Left if x > 0 => Some((x - 1, y)),
            Neighbour::Right if x < width - 1 => Some((x + 1, y)),
            Neighbour::Up if y > 0 => Some((x, y - 1)),
            Neighbour::Down if y < height - 1 => Some((x, y + 1)),
            Neighbour::UpperLeft => Neighbour::Left
                .apply(x, y, width, height)
                .and_then(|(x, y)| Neighbour::Up.apply(x, y, width, height)),
            Neighbour::UpperRight => Neighbour::Right
                .apply(x, y, width, height)
                .and_then(|(x, y)| Neighbour::Up.apply(x, y, width, height)),
            Neighbour::LowerLeft => Neighbour::Left
                .apply(x, y, width, height)
                .and_then(|(x, y)| Neighbour::Down.apply(x, y, width, height)),
            Neighbour::LowerRight => Neighbour::Right
                .apply(x, y, width, height)
                .and_then(|(x, y)| Neighbour::Down.apply(x, y, width, height)),
            _ => None,
        }
    }

    const ALL: &'static [Self] = &[
        Neighbour::Left,
        Neighbour::Right,
        Neighbour::Up,
        Neighbour::Down,
        Neighbour::UpperLeft,
        Neighbour::UpperRight,
        Neighbour::LowerLeft,
        Neighbour::LowerRight,
    ];
}

#[derive(Debug)]
struct OctopusGrid {
    octopuses: Vec<Octopus>,
    width: usize,
    height: usize,
}

impl OctopusGrid {
    fn neighbours(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> + 'static {
        let width = self.width;
        let height = self.height;
        Neighbour::ALL
            .iter()
            .copied()
            .filter_map(move |neighbour| neighbour.apply(x, y, width, height))
    }

    fn get_mut(&mut self, x: usize, y: usize) -> &mut Octopus {
        &mut self.octopuses[y * self.width + x]
    }

    fn step(&mut self) -> usize {
        // First, the energy level of each octopus increases by 1.
        self.octopuses
            .iter_mut()
            .for_each(Octopus::increment_energy_level);

        // Then, any octopus with an energy level greater than 9 flashes. This increases the energy
        // level of all adjacent octopuses by 1, including octopuses that are diagonally adjacent.
        // If this causes an octopus to have an energy level greater than 9, it also flashes. This
        // process continues as long as new octopuses keep having their energy level increased
        // beyond 9. (An octopus can only flash at most once per step.)
        let mut flashed = true;
        let mut flashes = 0;
        while flashed {
            flashed = false;
            for index in 0..self.octopuses.len() {
                if self.octopuses[index].flashes() {
                    let (x, y) = (index % self.width, index / self.width);
                    for (x, y) in self.neighbours(x, y) {
                        self.get_mut(x, y).increment_energy_level();
                    }

                    flashed = true;
                    flashes += 1;
                }
            }
        }

        // Finally, any octopus that flashed during this step has its energy level set to 0, as it
        // used all of its energy to flash.
        self.octopuses
            .iter_mut()
            .for_each(Octopus::reset_energy_level);

        flashes
    }
}

impl std::str::FromStr for OctopusGrid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut octopuses = Vec::new();

        let mut height = 0;
        let mut width = None;
        for line in s.lines().map(str::trim) {
            let octopus_line = line
                .chars()
                .map(TryFrom::try_from)
                .collect::<Result<Vec<_>>>()?;

            if width.is_none() {
                width = Some(octopus_line.len());
            }

            height += 1;
            octopuses.extend_from_slice(&octopus_line);
        }

        Ok(OctopusGrid {
            octopuses,
            width: width.context("0 lines parsed, width never computed")?,
            height,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED: &str = include_str!("../input/day11_provided.txt");

    #[test]
    fn part1_provided() {
        assert_eq!(part1(PROVIDED).unwrap(), 1656);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 1588);
    }

    #[test]
    fn part2_provided() {
        assert_eq!(part2(PROVIDED).unwrap(), 195);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT).unwrap(), 517);
    }
}
