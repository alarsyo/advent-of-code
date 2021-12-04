use std::collections::HashSet;
use std::fmt::Write;

use anyhow::{bail, Context, Result};

const INPUT: &str = include_str!("../input/day24.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;
    writeln!(res, "part 2: {}", part2(INPUT)?)?;

    Ok(res)
}

fn compute_pattern(input: &str) -> Result<HashSet<HexCoordinates>> {
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

    Ok(black_tiles)
}

fn part1(input: &str) -> Result<usize> {
    let black_tiles = compute_pattern(input)?;
    Ok(black_tiles.len())
}

fn part2(input: &str) -> Result<usize> {
    let mut black_tiles = compute_pattern(input)?;

    for _ in 0..100 {
        let mut new_black_tiles = black_tiles.clone();

        let mut seen = HashSet::new();
        let mut todo: Vec<HexCoordinates> = black_tiles.iter().copied().collect();

        while !todo.is_empty() {
            let tile = todo.pop().unwrap();

            if seen.contains(&tile) {
                continue;
            } else {
                seen.insert(tile);
            }

            let neighbours = tile.neighbours();

            let count = neighbours
                .iter()
                .filter(|tile| black_tiles.contains(tile))
                .count();
            if black_tiles.contains(&tile) {
                // Any black tile with zero or more than 2 black tiles immediately adjacent to it is
                // flipped to white.
                if count == 0 || count > 2 {
                    new_black_tiles.remove(&tile);
                }
            } else {
                // Any white tile with exactly 2 black tiles immediately adjacent to it is flipped
                // to black.
                if count == 2 {
                    new_black_tiles.insert(tile);
                }
            }

            if black_tiles.contains(&tile) {
                for n in &neighbours {
                    todo.push(*n);
                }
            }
        }

        black_tiles = new_black_tiles;
    }

    Ok(black_tiles.len())
}

/// Hexagonal tile coordinate representation
///
/// These use the axial coordinates described here:
///
/// https://www.redblobgames.com/grids/hexagons/#coordinates-axial
#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
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

    fn neighbours(self) -> [Self; 6] {
        [
            self.east(),
            self.west(),
            self.north_west(),
            self.north_east(),
            self.south_east(),
            self.south_west(),
        ]
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

    #[test]
    #[ignore]
    fn part2_provided() {
        assert_eq!(part2(PROVIDED).unwrap(), 2208);
    }

    #[test]
    #[ignore]
    fn part2_real() {
        assert_eq!(part2(INPUT).unwrap(), 4200);
    }
}
