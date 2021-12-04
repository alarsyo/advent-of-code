use std::collections::HashMap;
use std::fmt::Write;

use anyhow::{anyhow, Context, Result};

const INPUT: &str = include_str!("../input/day04.txt");

const GRID_WIDTH: usize = 5;
const GRID_HEIGHT: usize = 5;
const GRID_SIZE: usize = GRID_WIDTH * GRID_HEIGHT;

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;
    writeln!(res, "part 2: {}", part2(INPUT)?)?;

    Ok(res)
}

fn part1(input: &str) -> Result<u64> {
    let (draws, grids) = input
        .split_once("\n\n")
        .context("couldn't split draws from grids")?;

    let draws = draws
        .split(',')
        .map(|num| num.parse::<u8>().context("couldn't parse drawn number:"))
        .collect::<Result<Vec<_>>>()?;
    let mut grids = grids
        .split("\n\n")
        .map(str::parse::<Grid>)
        .collect::<Result<Vec<_>>>()?;

    let (mut wdraw, mut wgrid) = (None, None);

    'draw_loop: for draw in draws {
        for grid in &mut grids {
            if grid.mark(draw) && grid.is_winning() {
                wgrid = Some(grid.clone());
                wdraw = Some(draw);
                break 'draw_loop;
            }
        }
    }

    match (wdraw, wgrid) {
        (Some(draw), Some(grid)) => {
            Ok(draw as u64 * grid.unmarked_numbers().map(|n| *n as u64).sum::<u64>())
        }
        _ => Err(anyhow!("couldn't find a winning grid!")),
    }
}

fn part2(input: &str) -> Result<u64> {
    let (draws, grids) = input
        .split_once("\n\n")
        .context("couldn't split draws from grids")?;

    let draws = draws
        .split(',')
        .map(|num| num.parse::<u8>().context("couldn't parse drawn number:"))
        .collect::<Result<Vec<_>>>()?;
    let mut grids = grids
        .split("\n\n")
        .map(str::parse::<Grid>)
        .collect::<Result<Vec<_>>>()?;

    let mut draws = draws.into_iter();

    while grids.len() > 1 {
        let draw = draws
            .next()
            .context("no draws available, didn't find last grid")?;

        // TODO: replace with drain_filter when stabilized
        let mut i = 0;
        while i < grids.len() {
            let grid = &mut grids[i];
            if grid.mark(draw) && grid.is_winning() {
                grids.remove(i);
            } else {
                i += 1;
            }
        }
    }

    let last_grid = &mut grids[0];

    for draw in draws {
        if last_grid.mark(draw) && last_grid.is_winning() {
            return Ok(draw as u64 * last_grid.unmarked_numbers().map(|n| *n as u64).sum::<u64>());
        }
    }

    Err(anyhow!("last grid never wins, this is not expected"))
}

#[derive(Debug, Clone)]
struct Grid {
    number_to_pos: HashMap<u8, (usize, usize)>,
    pos_to_number: HashMap<(usize, usize), u8>,
    grid: [bool; GRID_SIZE],
}

impl Grid {
    fn mark(&mut self, draw: u8) -> bool {
        match self.number_to_pos.get(&draw) {
            Some(&(x, y)) => {
                *self.access_grid_mut(x, y) = true;
                true
            }
            None => false,
        }
    }

    fn is_winning(&self) -> bool {
        let mut rows = [0u8; GRID_HEIGHT];
        let mut cols = [0u8; GRID_WIDTH];

        for (y, row) in rows.iter_mut().enumerate() {
            for (x, col) in cols.iter_mut().enumerate() {
                if self.access_grid(x, y) {
                    *row += 1;
                    *col += 1;

                    if *row as usize == GRID_WIDTH || *col as usize == GRID_HEIGHT {
                        return true;
                    }
                }
            }
        }

        false
    }

    fn unmarked_numbers(&self) -> impl Iterator<Item = &u8> {
        self.number_to_pos
            .iter()
            .filter_map(|(num, &(x, y))| (!self.access_grid(x, y)).then(|| num))
    }

    fn access_grid(&self, x: usize, y: usize) -> bool {
        self.grid[y * GRID_HEIGHT + x]
    }

    fn access_grid_mut(&mut self, x: usize, y: usize) -> &mut bool {
        &mut self.grid[y * GRID_HEIGHT + x]
    }
}

impl std::str::FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut numbers = s.split_whitespace().map(str::parse);
        let mut number_to_pos = HashMap::new();
        let mut pos_to_number = HashMap::new();

        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let pos = (x, y);
                let number: u8 = numbers
                    .next()
                    .context("not enough numbers for grid")?
                    .context("couldn't parse number:")?;
                number_to_pos.insert(number, pos);
                pos_to_number.insert(pos, number);
            }
        }

        Ok(Grid {
            number_to_pos,
            pos_to_number,
            grid: [false; GRID_SIZE],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED: &str = include_str!("../input/day04_provided.txt");

    #[test]
    fn part1_provided() {
        assert_eq!(part1(PROVIDED).unwrap(), 4512);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 45031);
    }

    #[test]
    fn part2_provided() {
        assert_eq!(part2(PROVIDED).unwrap(), 1924);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT).unwrap(), 2568);
    }
}