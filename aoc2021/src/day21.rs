use std::fmt::Write;
use std::iter;
use std::ops::RangeInclusive;

use anyhow::{Context, Result};

const INPUT: &str = include_str!("../input/day21.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;

    Ok(res)
}

fn part1(input: &str) -> Result<usize> {
    let mut lines = input.lines();
    let mut player1_pos: PlayerPos = lines
        .next()
        .and_then(|line| line.trim().strip_prefix("Player 1 starting position: "))
        .and_then(|pos| pos.parse().ok())
        .map(PlayerPos::new)
        .context("couldn't find player 1 pos")?;
    let mut player2_pos: PlayerPos = lines
        .next()
        .and_then(|line| line.trim().strip_prefix("Player 2 starting position: "))
        .and_then(|pos| pos.parse().ok())
        .map(PlayerPos::new)
        .context("couldn't find player 2 pos")?;

    let mut player1_score = 0;
    let mut player2_score = 0;
    let mut dice = DeterministicDice::new();

    while player2_score < 1000 {
        let mv = dice.next_3_sum();

        player1_pos.advance_by(mv);
        player1_score += player1_pos.pos();

        std::mem::swap(&mut player1_pos, &mut player2_pos);
        std::mem::swap(&mut player1_score, &mut player2_score);
    }

    let loser_score = player1_score;

    Ok(loser_score * dice.rolls())
}

struct PlayerPos(usize);

impl PlayerPos {
    fn new(pos: usize) -> Self {
        debug_assert!((1..=10).contains(&pos));

        // represented from 0 to 9 for modulo ease of use
        Self(pos - 1)
    }

    fn advance_by(&mut self, mv: usize) {
        self.0 = (self.0 + mv) % 10
    }

    fn pos(&self) -> usize {
        self.0 + 1
    }
}

struct DeterministicDice {
    iter: iter::Cycle<RangeInclusive<usize>>,
    rolls: usize,
}

impl DeterministicDice {
    fn new() -> Self {
        Self {
            iter: (1..=100).cycle(),
            rolls: 0,
        }
    }

    fn next_3_sum(&mut self) -> usize {
        self.rolls += 3;

        self.iter.next().unwrap() + self.iter.next().unwrap() + self.iter.next().unwrap()
    }

    fn rolls(&self) -> usize {
        self.rolls
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED: &str = include_str!("../input/day21_provided.txt");

    #[test]
    fn part1_provided() {
        assert_eq!(part1(PROVIDED).unwrap(), 739785);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 908595);
    }
}
