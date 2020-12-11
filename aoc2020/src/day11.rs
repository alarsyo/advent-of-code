use std::fmt::Write;

use aoc::err;

const INPUT: &str = include_str!("../input/day11.txt");

pub fn run() -> aoc::Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;
    writeln!(res, "part 2: {}", part2(INPUT)?)?;

    Ok(res)
}

fn part1(input: &str) -> aoc::Result<usize> {
    let mut layout: Layout = input.parse()?;

    let occupied_threshold = 4;
    layout.converge(occupied_threshold, Layout::count_adjacent);

    Ok(layout.occupied_seats())
}

fn part2(input: &str) -> aoc::Result<usize> {
    let mut layout: Layout = input.parse()?;

    let occupied_threshold = 5;
    layout.converge(occupied_threshold, Layout::count_line_of_sight);

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
    fn step(
        &mut self,
        occupied_threshold: u8,
        adj_count: fn(&Self, usize, usize, Cell) -> u8,
    ) -> bool {
        let grid = &self.grid;

        let mut new = grid.clone();
        let mut changed = false;

        for i in 0..self.height {
            for j in 0..self.width {
                let cell = self[i][j];

                match cell {
                    Cell::EmptySeat => {
                        if adj_count(&self, i, j, Cell::OccupiedSeat) == 0 {
                            new[i][j] = Cell::OccupiedSeat;
                            changed = true;
                        }
                    }
                    Cell::OccupiedSeat => {
                        if adj_count(&self, i, j, Cell::OccupiedSeat) >= occupied_threshold {
                            new[i][j] = Cell::EmptySeat;
                            changed = true;
                        }
                    }
                    _ => {}
                }
            }
        }

        self.grid = new;

        changed
    }

    /// Steps through the simulation until a fixpoint is reached
    fn converge(&mut self, occupied_threshold: u8, adj_count: fn(&Self, usize, usize, Cell) -> u8) {
        while self.step(occupied_threshold, adj_count) {}
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

    fn count_line_of_sight(&self, i: usize, j: usize, value: Cell) -> u8 {
        let mut count = 0;

        for (di, dj) in Self::OFFSETS {
            let mut distance = 1;

            let diff = loop {
                let (di, dj) = (di * distance, dj * distance);

                let (i, j) = (i.wrapping_add(di as usize), j.wrapping_add(dj as usize));

                let cell = self.grid.get(i).map(|line| line.get(j)).flatten();

                match cell {
                    // keep going, the next seat is farther away
                    Some(Cell::Floor) => distance += 1,
                    // found the kind of seat we care about
                    Some(&seat) if seat == value => break 1,
                    // found a seat that blocks line of sight, or reached out of bounds
                    Some(_) | None => break 0,
                }
            };

            // only check seat if it's in bounds
            count += diff;
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

    #[test]
    fn part2_provided() {
        assert_eq!(part2(PROVIDED).unwrap(), 26);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT).unwrap(), 2199);
    }
}
