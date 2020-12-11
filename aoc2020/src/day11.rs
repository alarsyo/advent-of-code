use std::fmt::Write;
use std::mem;

use aoc::err;

const INPUT: &str = include_str!("../input/day11.txt");

pub fn run() -> aoc::Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;

    Ok(res)
}

fn part1(input: &str) -> aoc::Result<usize> {
    let mut layout: Layout = input.parse()?;

    layout.converge();

    Ok(layout.occupied_seats())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    EmptySeat,
    Floor,
    OccupiedSeat,
}

type Grid = Vec<Vec<Cell>>;

struct Layout {
    grid: Grid,
    height: usize,
    width: usize,
}

impl Layout {
    /// Steps one round in the simulation, returns the previous grid
    fn step(&mut self) -> Grid {
        let grid = &self.grid;

        let mut new = grid.clone();

        for i in 0..self.height {
            for j in 0..self.width {
                let cell = self[i][j];

                match cell {
                    Cell::EmptySeat => {
                        if self.count_adjacent(i, j, Cell::OccupiedSeat) == 0 {
                            new[i][j] = Cell::OccupiedSeat;
                        }
                    }
                    Cell::OccupiedSeat => {
                        if self.count_adjacent(i, j, Cell::OccupiedSeat) >= 4 {
                            new[i][j] = Cell::EmptySeat;
                        }
                    }
                    _ => {}
                }
            }
        }

        mem::replace(&mut self.grid, new)
    }

    /// Steps through the simulation until a fixpoint is reached
    fn converge(&mut self) {
        let mut prev = self.step();

        while prev != self.grid {
            prev = self.step();
        }
    }

    const OFFSETS: &'static [(i8, i8)] = &[
        (0, -1),
        (0, 1),
        (-1, 0),
        (1, 0),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ];

    fn count_adjacent(&self, i: usize, j: usize, value: Cell) -> u8 {
        let mut count = 0;

        for (di, dj) in Self::OFFSETS {
            let (i, j) = (i.wrapping_add(*di as usize), j.wrapping_add(*dj as usize));

            // only check seat if it's in bounds
            count += self
                .grid
                .get(i)
                .map(|line| line.get(j))
                .flatten()
                .map(|&cell| if cell == value { 1 } else { 0 })
                .unwrap_or(0);
        }

        count
    }

    fn occupied_seats(&self) -> usize {
        self.grid
            .iter()
            .map(|line| {
                line.iter()
                    .filter(|seat| **seat == Cell::OccupiedSeat)
                    .count()
            })
            .sum()
    }
}

impl std::ops::Index<usize> for Layout {
    type Output = Vec<Cell>;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.grid[idx]
    }
}

impl std::ops::IndexMut<usize> for Layout {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.grid[idx]
    }
}

impl std::str::FromStr for Layout {
    type Err = aoc::Error;

    fn from_str(s: &str) -> aoc::Result<Self> {
        let grid = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => Ok(Cell::Floor),
                        'L' => Ok(Cell::EmptySeat),
                        '#' => Ok(Cell::OccupiedSeat),
                        _ => Err(err!("unknown char `{}`", c)),
                    })
                    .collect()
            })
            .collect::<aoc::Result<Grid>>()?;

        let height = grid.len();
        let width = grid[0].len();

        Ok(Self {
            grid,
            height,
            width,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED: &str = include_str!("../input/day11_provided.txt");

    #[test]
    fn part1_provided() {
        assert_eq!(part1(PROVIDED).unwrap(), 37);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 2427);
    }
}
