use std::{fmt::Write, str::FromStr};

use anyhow::{bail, Result};

const INPUT: &str = include_str!("../input/day01.txt");
const DIAL_SIZE: u16 = 100;

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);
    writeln!(res, "part 1: {}", part1(INPUT)?)?;
    writeln!(res, "part 2: {}", part2(INPUT)?)?;
    Ok(res)
}

enum Rotation {
    Left(u16),
    Right(u16),
}

impl FromStr for Rotation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let dir = &s[..1];
        let num = s[1..].parse()?;

        match dir {
            "L" => Ok(Rotation::Left(num)),
            "R" => Ok(Rotation::Right(num)),
            _ => bail!("rotation can only be left or right, got `{}'", dir),
        }
    }
}

impl Rotation {
    fn apply(&self, state: u16) -> u16 {
        match self {
            Rotation::Left(num) => {
                let num = num % DIAL_SIZE;
                if num > state {
                    DIAL_SIZE - (num - state)
                } else {
                    state - num
                }
            }
            Rotation::Right(num) => (state + num) % DIAL_SIZE,
        }
    }

    /// Counts the number of zeroes encountered while rotating the dial. Does NOT take the initial
    /// state into account if it is 0.
    fn apply_and_count_zeroes(&self, state: u16) -> (u16, usize) {
        match self {
            Rotation::Left(num) => {
                let zeroes = (num / DIAL_SIZE) as usize;
                let num = num % DIAL_SIZE;
                if num > state {
                    (
                        DIAL_SIZE - (num - state),
                        zeroes + if state != 0 { 1 } else { 0 },
                    )
                } else {
                    let new_state = state - num;
                    (new_state, zeroes + if new_state == 0 { 1 } else { 0 })
                }
            }
            Rotation::Right(num) => (
                (state + num) % DIAL_SIZE,
                ((state + num) / DIAL_SIZE) as usize,
            ),
        }
    }
}

fn part1(input: &str) -> Result<usize> {
    let rotations = input
        .lines()
        .map(Rotation::from_str)
        .collect::<Result<Vec<Rotation>>>()?;

    let res = rotations
        .iter()
        .fold((50, 0), |(state, mut count), rot| {
            let new_state = rot.apply(state);
            if new_state == 0 {
                count += 1;
            }
            (new_state, count)
        })
        .1;
    Ok(res)
}

fn part2(input: &str) -> Result<usize> {
    let rotations = input
        .lines()
        .map(Rotation::from_str)
        .collect::<Result<Vec<Rotation>>>()?;

    let res = rotations
        .iter()
        .fold((50, 0), |(state, count), rot| {
            let (new_state, zeroes) = rot.apply_and_count_zeroes(state);
            (new_state, count + zeroes)
        })
        .1;
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED: &str = include_str!("../input/day01_provided.txt");

    #[test]
    fn part1_provided() {
        assert_eq!(part1(PROVIDED).unwrap(), 3);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 1092);
    }

    #[test]
    fn part2_provided() {
        assert_eq!(part2(PROVIDED).unwrap(), 6);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT).unwrap(), 6616);
    }
}
