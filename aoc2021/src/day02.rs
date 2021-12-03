use std::fmt::Write;

use anyhow::{bail, Context, Result};

const INPUT: &str = include_str!("../input/day02.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;
    writeln!(res, "part 2: {}", part2(INPUT)?)?;

    Ok(res)
}

fn part1(input: &str) -> Result<u64> {
    let commands = input
        .lines()
        .map(str::parse::<Command>)
        .collect::<Result<Vec<_>>>()?;
    let mut horizontal_pos = 0;
    let mut depth = 0;

    for c in commands {
        match c {
            Command::Forward(dx) => horizontal_pos += dx,
            Command::Up(dz) => depth -= dz,
            Command::Down(dz) => depth += dz,
        }
    }

    Ok(depth * horizontal_pos)
}

fn part2(input: &str) -> Result<u64> {
    let commands = input
        .lines()
        .map(str::parse::<Command>)
        .collect::<Result<Vec<_>>>()?;
    let mut horizontal_pos = 0;
    let mut depth = 0;
    let mut aim = 0;

    for c in commands {
        match c {
            Command::Forward(dx) => {
                horizontal_pos += dx;
                depth += aim * dx;
            }
            Command::Up(dz) => {
                aim -= dz;
            }
            Command::Down(dz) => {
                aim += dz;
            }
        }
    }

    Ok(depth * horizontal_pos)
}

enum Command {
    Forward(u64),
    Up(u64),
    Down(u64),
}

impl std::str::FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (word, number) = s.split_once(' ').context("couldn't split command")?;

        let number = number.parse()?;

        Ok(match word {
            "forward" => Self::Forward(number),
            "up" => Self::Up(number),
            "down" => Self::Down(number),
            _ => bail!("unkown command"),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED: &str = include_str!("../input/day02_provided.txt");

    #[test]
    fn part1_provided() {
        assert_eq!(part1(PROVIDED).unwrap(), 150);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 1962940);
    }

    #[test]
    fn part2_provided() {
        assert_eq!(part2(PROVIDED).unwrap(), 900);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT).unwrap(), 1813664422);
    }
}
