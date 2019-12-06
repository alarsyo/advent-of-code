use std::error::Error;
use std::fmt::Write;
use std::str::FromStr;

use aoc::err;
use aoc::Result;

const INPUT: &str = include_str!("../input/day06.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;
    writeln!(res, "part 2: {}", part2(INPUT)?)?;

    Ok(res)
}

fn part1(input: &str) -> Result<usize> {
    let instructions = input
        .lines()
        .map(|line| {
            line.trim_end()
                .parse()
                .map_err(|e| err!("couldn't parse instruction: {}", e))
        })
        .collect::<Result<Vec<Instruction>>>()?;

    let mut grid: Vec<Vec<bool>> = vec![vec![false; 1000]; 1000];

    for inst in instructions {
        match inst.action {
            Action::TurnOn => {
                for (x, y) in inst {
                    grid[x][y] = true;
                }
            }
            Action::TurnOff => {
                for (x, y) in inst {
                    grid[x][y] = false;
                }
            }
            Action::Toggle => {
                for (x, y) in inst {
                    grid[x][y] = !grid[x][y];
                }
            }
        }
    }

    Ok(grid
        .iter()
        .flat_map(|line| line.iter())
        .filter(|v| **v)
        .count())
}

fn part2(input: &str) -> Result<u64> {
    let instructions = input
        .lines()
        .map(|line| {
            line.trim_end()
                .parse()
                .map_err(|e| err!("couldn't parse instruction: {}", e))
        })
        .collect::<Result<Vec<Instruction>>>()?;

    let mut grid: Vec<Vec<u64>> = vec![vec![0; 1000]; 1000];

    for inst in instructions {
        match inst.action {
            Action::TurnOn => {
                for (x, y) in inst {
                    grid[x][y] = grid[x][y].saturating_add(1);
                }
            }
            Action::TurnOff => {
                for (x, y) in inst {
                    grid[x][y] = grid[x][y].saturating_sub(1);
                }
            }
            Action::Toggle => {
                for (x, y) in inst {
                    grid[x][y] = grid[x][y].saturating_add(2);
                }
            }
        }
    }

    Ok(grid.iter().flat_map(|line| line.iter()).sum())
}

enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}

struct Instruction {
    action: Action,
    horizontal: (usize, usize),
    vertical: (usize, usize),
}

impl IntoIterator for Instruction {
    type Item = (usize, usize);
    type IntoIter = Box<dyn Iterator<Item = Self::Item>>;

    fn into_iter(self) -> Self::IntoIter {
        let begin_x = self.horizontal.0;
        let end_x = self.horizontal.1;
        let begin_y = self.vertical.0;
        let end_y = self.vertical.1;

        let iter = (begin_x..=end_x).flat_map(move |i| (begin_y..=end_y).map(move |j| (i, j)));

        Box::new(iter)
    }
}

impl FromStr for Instruction {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        let mut space = s
            .find(' ')
            .ok_or_else(|| err!("couldn't parse instruction: {}", s))?;

        let action = if &s[..space] == "toggle" {
            Action::Toggle
        } else if &s[..space] == "turn" {
            space = s[(space + 1)..]
                .find(' ')
                .ok_or_else(|| err!("couldn't parse instruction: {}", s))?
                + space
                + 1;
            if &s[..space] == "turn on" {
                Action::TurnOn
            } else if &s[..space] == "turn off" {
                Action::TurnOff
            } else {
                return Err(err!("couldn't identify action: {}", &s[..space]));
            }
        } else {
            return Err(err!("couldn't identify action: {}", s));
        };
        let s = &s[(space + 1)..];

        let comma = s
            .find(',')
            .ok_or_else(|| err!("couldn't parse instruction: {}", s))?;
        let x = s[..comma].parse()?;
        let s = &s[(comma + 1)..];

        let space = s
            .find(' ')
            .ok_or_else(|| err!("couldn't parse instruction: {}", s))?;
        let y = s[..space].parse()?;
        let s = &s[(space + 1)..];

        let begin = (x, y);

        let through = s
            .find("through ")
            .ok_or_else(|| err!("couldn't parse instruction: {}", s))?;
        let s = &s[(through + 8)..];

        let comma = s
            .find(',')
            .ok_or_else(|| err!("couldn't parse instruction: {}", s))?;
        let x = s[..comma].parse()?;
        let s = &s[(comma + 1)..];

        let y = s.parse()?;

        let end = (x, y);

        Ok(Instruction {
            action,
            horizontal: (begin.0, end.0),
            vertical: (begin.1, end.1),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_provided() {
        assert_eq!(part1("turn on 0,0 through 999,999").unwrap(), 1_000_000);
        assert_eq!(part1("toggle 0,0 through 999,0").unwrap(), 1_000);
        assert_eq!(part1("turn off 499,499 through 500,500").unwrap(), 0);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 543903);
    }

    #[test]
    fn part2_provided() {
        assert_eq!(part2("turn on 0,0 through 0,0").unwrap(), 1);
        assert_eq!(part2("toggle 0,0 through 999,999").unwrap(), 2_000_000);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT).unwrap(), 14687245);
    }
}
