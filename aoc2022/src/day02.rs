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
    let mut score = 0;
    for line in input.lines() {
        let round: RoundPart1 = line.parse()?;
        score += round.score();
    }

    Ok(score)
}

fn part2(input: &str) -> Result<u64> {
    let mut score = 0;
    for line in input.lines() {
        let round: RoundPart2 = line.parse()?;
        score += round.score();
    }

    Ok(score)
}

#[derive(Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl std::str::FromStr for Shape {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => bail!("unsupported shape encoding: {}", s),
        }
    }
}

impl Shape {
    fn score(&self) -> u64 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

struct RoundPart1 {
    opponent_move: Shape,
    my_move: Shape,
}

impl std::str::FromStr for RoundPart1 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (opponent_move, my_move) =
            s.split_once(' ').context("couldn't split round on space")?;
        let opponent_move = opponent_move.parse()?;
        let my_move = my_move.parse()?;

        Ok(Self {
            opponent_move,
            my_move,
        })
    }
}

impl RoundPart1 {
    fn outcome(&self) -> Outcome {
        match (&self.opponent_move, &self.my_move) {
            (Shape::Rock, Shape::Rock) => Outcome::Draw,
            (Shape::Rock, Shape::Paper) => Outcome::Won,
            (Shape::Rock, Shape::Scissors) => Outcome::Lost,
            (Shape::Paper, Shape::Rock) => Outcome::Lost,
            (Shape::Paper, Shape::Paper) => Outcome::Draw,
            (Shape::Paper, Shape::Scissors) => Outcome::Won,
            (Shape::Scissors, Shape::Rock) => Outcome::Won,
            (Shape::Scissors, Shape::Paper) => Outcome::Lost,
            (Shape::Scissors, Shape::Scissors) => Outcome::Draw,
        }
    }

    fn score(&self) -> u64 {
        self.outcome().score() + self.my_move.score()
    }
}

struct RoundPart2 {
    opponent_move: Shape,
    outcome: Outcome,
}

impl std::str::FromStr for RoundPart2 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (opponent_move, outcome) =
            s.split_once(' ').context("couldn't split round on space")?;
        let opponent_move = opponent_move.parse()?;
        let outcome = outcome.parse()?;

        Ok(Self {
            opponent_move,
            outcome,
        })
    }
}

impl RoundPart2 {
    fn shape_to_play(&self) -> Shape {
        match (&self.opponent_move, &self.outcome) {
            (Shape::Rock, Outcome::Lost) => Shape::Scissors,
            (Shape::Rock, Outcome::Draw) => Shape::Rock,
            (Shape::Rock, Outcome::Won) => Shape::Paper,
            (Shape::Paper, Outcome::Lost) => Shape::Rock,
            (Shape::Paper, Outcome::Draw) => Shape::Paper,
            (Shape::Paper, Outcome::Won) => Shape::Scissors,
            (Shape::Scissors, Outcome::Lost) => Shape::Paper,
            (Shape::Scissors, Outcome::Draw) => Shape::Scissors,
            (Shape::Scissors, Outcome::Won) => Shape::Rock,
        }
    }

    fn score(&self) -> u64 {
        self.outcome.score() + self.shape_to_play().score()
    }
}

enum Outcome {
    Lost,
    Draw,
    Won,
}

impl std::str::FromStr for Outcome {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Lost),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Won),
            _ => bail!("unsupported outcome encoding: {}", s),
        }
    }
}

impl Outcome {
    fn score(&self) -> u64 {
        match self {
            Self::Lost => 0,
            Self::Draw => 3,
            Self::Won => 6,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED: &str = include_str!("../input/day02_provided.txt");

    #[test]
    fn part1_provided() {
        assert_eq!(part1(PROVIDED).unwrap(), 15);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 11150);
    }

    #[test]
    fn part2_provided() {
        assert_eq!(part2(PROVIDED).unwrap(), 12);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT).unwrap(), 8295);
    }
}
