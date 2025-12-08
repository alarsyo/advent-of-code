use std::{fmt::Write, str::FromStr};

use anyhow::{Result, bail};

const INPUT: &str = include_str!("../input/day04.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);
    writeln!(res, "part 1: {}", part1(INPUT)?)?;
    writeln!(res, "part 2: {}", part2(INPUT)?)?;
    Ok(res)
}

struct PaperRollsMap(Vec<Vec<bool>>);

impl FromStr for PaperRollsMap {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut grid = Vec::new();

        let mut width = None;
        for line in s.lines() {
            match width {
                Some(size) => assert_eq!(size, line.len()),
                None => width = Some(line.len()),
            }

            grid.push(
                line.chars()
                    .map(|c| match c {
                        '@' => Ok(true),
                        '.' => Ok(false),
                        _ => bail!("unknown character `{}' while parsing grid", c),
                    })
                    .collect::<Result<Vec<_>>>()?,
            );
        }

        Ok(Self(grid))
    }
}

impl PaperRollsMap {
    fn width(&self) -> usize {
        self.0[0].len()
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn count_accessible_rolls(&self) -> usize {
        let mut count = 0;
        for y in 0..self.len() {
            for x in 0..self.width() {
                if self.0[y][x] && self.roll_is_accessible(x, y) {
                    count += 1
                }
            }
        }
        count
    }

    fn roll_is_accessible(&self, x: usize, y: usize) -> bool {
        let mut count = 0;
        for dy in [-1, 0, 1] {
            match y.checked_add_signed(dy) {
                None => continue,
                Some(yval) if yval == self.len() => continue,
                Some(yval) => {
                    for dx in [-1, 0, 1] {
                        match x.checked_add_signed(dx) {
                            None => continue,
                            Some(xval) if xval == self.width() => continue,
                            Some(xval) => {
                                if self.0[yval][xval] {
                                    count += 1
                                }
                            }
                        }
                    }
                }
            }
        }
        count <= 4
    }

    fn count_removable_rolls(&mut self) -> usize {
        let mut count = 0;
        loop {
            let mut cur_loop_count = 0;
            let mut copy = PaperRollsMap(self.0.clone());
            for y in 0..self.len() {
                for x in 0..self.width() {
                    if self.0[y][x] && self.roll_is_accessible(x, y) {
                        copy.0[y][x] = false;
                        cur_loop_count += 1;
                    }
                }
            }

            if cur_loop_count == 0 {
                break;
            }

            *self = copy;
            count += cur_loop_count;
        }
        count
    }
}

fn part1(input: &str) -> Result<usize> {
    let grid: PaperRollsMap = input.parse()?;

    Ok(grid.count_accessible_rolls())
}

fn part2(input: &str) -> Result<usize> {
    let mut grid: PaperRollsMap = input.parse()?;

    Ok(grid.count_removable_rolls())
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED: &str = include_str!("../input/day04_provided.txt");

    #[test]
    fn part1_provided() {
        assert_eq!(part1(PROVIDED).unwrap(), 13);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 1393);
    }

    #[test]
    fn part2_provided() {
        assert_eq!(part2(PROVIDED).unwrap(), 43);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT).unwrap(), 8643);
    }
}
