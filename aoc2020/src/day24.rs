use std::collections::HashSet;
use std::fmt::Write;

use anyhow::{bail, Context, Result};

const INPUT: &str = include_str!("../input/day24.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;

    Ok(res)
}

fn part1(input: &str) -> Result<usize> {
    let mut black_tiles = HashSet::new();

    for line in input.lines() {
        let mut line = line;
        let mut tile_pos = HexCoordinates::default();

        // compute tile coordinates by going through the whole line
        while !line.is_empty() {
            match line.chars().next().unwrap() {
                'e' => {
                    tile_pos = tile_pos.east();
                    line = &line[1..];
                }
                'w' => {
                    tile_pos = tile_pos.west();
                    line = &line[1..];
                }
                'n' => {
                    match line
                        .chars()
                        .nth(1)
                        .context("invalid input, missing char after `n`")?
                    {
                        'e' => tile_pos = tile_pos.north_east(),
                        'w' => tile_pos = tile_pos.north_west(),
                        other => bail!("unexpected character in input: `{}`", other),
                    }

                    line = &line[2..];
                }
                's' => {
                    match line
                        .chars()
                        .nth(1)
                        .context("invalid input, missing char after `s`")?
                    {
                        'e' => tile_pos = tile_pos.south_east(),
                        'w' => tile_pos = tile_pos.south_west(),
                        other => bail!("unexpected character in input: `{}`", other),
                    }

                    line = &line[2..];
                }
                other => bail!("unexpected character in input: `{}`", other),
            }
        }

        if black_tiles.contains(&tile_pos) {
            black_tiles.remove(&tile_pos);
        } else {
            black_tiles.insert(tile_pos);
        }
    }

    Ok(black_tiles.len())
}

/// Hexagonal tile coordinate representation
///
/// These use the axial coordinates described here:
///
/// https://www.redblobgames.com/grids/hexagons/#coordinates-axial
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct HexCoordinates {
    q: i64,
    r: i64,
}

impl HexCoordinates {
    fn east(self) -> Self {
        Self {
            q: self.q + 1,
            r: self.r,
        }
    }

    fn west(self) -> Self {
        Self {
            q: self.q - 1,
            r: self.r,
        }
    }

    fn north_west(self) -> Self {
        Self {
            q: self.q,
            r: self.r - 1,
        }
    }

    fn south_east(self) -> Self {
        Self {
            q: self.q,
            r: self.r + 1,
        }
    }

    fn north_east(self) -> Self {
        self.north_west().east()
    }

    fn south_west(self) -> Self {
        self.south_east().west()
    }
}

impl Default for HexCoordinates {
    fn default() -> Self {
        Self { q: 0, r: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED: &str = include_str!("../input/day24_provided.txt");

    #[test]
    fn part1_provided() {
        assert_eq!(part1(PROVIDED).unwrap(), 10);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 528);
    }
}
