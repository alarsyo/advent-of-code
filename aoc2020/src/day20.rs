use std::fmt::Write;

use anyhow::{anyhow, Context, Result};

const INPUT: &str = include_str!("../input/day20.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;

    Ok(res)
}

fn part1(input: &str) -> Result<u64> {
    let tiles: Vec<Tile> = input.split("\n\n").map(str::parse).collect::<Result<_>>()?;

    Ok(tiles
        .iter()
        .filter_map(|tile| {
            let mut count = 0;
            for other in &tiles {
                if tile == other {
                    continue;
                }

                count += tile
                    .edges
                    .iter()
                    .filter(|e| other.edges.contains(e))
                    .count();

                count += tile
                    .reversed_edges
                    .iter()
                    .filter(|e| other.edges.contains(e))
                    .count();
            }

            // corners have 2 edges in common
            if count == 2 {
                Some(tile.id)
            } else {
                None
            }
        })
        .product())
}

#[derive(Debug)]
struct Tile {
    id: u64,
    edges: [Vec<bool>; 4],
    reversed_edges: [Vec<bool>; 4],
}

impl std::cmp::PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl std::str::FromStr for Tile {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        const LINE_LENGTH: usize = 10;

        let mut lines = s.lines();

        let title = lines.next().context("couldn't find line with tile ID")?;
        let space = title.find(' ').unwrap();
        let colon = title.find(':').unwrap();
        let id = title[(space + 1)..colon].parse()?;

        let mut edges = [vec![], vec![], vec![], vec![]];

        lines
            .enumerate()
            .try_for_each::<_, Result<()>>(|(i, line)| {
                line.chars().enumerate().try_for_each(|(j, c)| {
                    let c = match c {
                        '#' => true,
                        '.' => false,
                        _ => return Err(anyhow!("unknown char `{}` while parsing tile", c)),
                    };

                    if i == 0 {
                        edges[0].push(c);
                    }
                    if j == 0 {
                        edges[1].push(c);
                    }
                    if i == (LINE_LENGTH - 1) {
                        edges[2].push(c);
                    }
                    if j == (LINE_LENGTH - 1) {
                        edges[3].push(c);
                    }

                    Ok(())
                })?;

                Ok(())
            })?;

        let mut reversed_edges = [vec![], vec![], vec![], vec![]];
        for (i, edge) in edges.iter().enumerate() {
            reversed_edges[i] = edge.iter().copied().rev().collect();
        }

        Ok(Tile {
            id,
            edges,
            reversed_edges,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED: &str = include_str!("../input/day20_provided.txt");

    #[test]
    fn part1_provided() {
        assert_eq!(part1(PROVIDED).unwrap(), 20_899_048_083_289);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 5_775_714_912_743);
    }
}
