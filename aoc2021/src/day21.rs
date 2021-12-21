use std::collections::HashMap;
use std::fmt::Write;
use std::iter;
use std::ops::RangeInclusive;

use anyhow::{Context, Result};

const INPUT: &str = include_str!("../input/day21.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;
    writeln!(res, "part 2: {}", part2(INPUT)?)?;

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

fn part2(input: &str) -> Result<usize> {
    let mut lines = input.lines();
    let player1_pos: PlayerPos = lines
        .next()
        .and_then(|line| line.trim().strip_prefix("Player 1 starting position: "))
        .and_then(|pos| pos.parse().ok())
        .map(PlayerPos::new)
        .context("couldn't find player 1 pos")?;
    let player2_pos: PlayerPos = lines
        .next()
        .and_then(|line| line.trim().strip_prefix("Player 2 starting position: "))
        .and_then(|pos| pos.parse().ok())
        .map(PlayerPos::new)
        .context("couldn't find player 2 pos")?;

    let (player1_score, player2_score) =
        quantum_dice_game(player1_pos, player2_pos, 0, 0, &mut HashMap::new());

    Ok(player1_score.max(player2_score))
}

type Cache = HashMap<(PlayerPos, PlayerPos, usize, usize), (usize, usize)>;

fn quantum_dice_game(
    pos1: PlayerPos,
    pos2: PlayerPos,
    score1: usize,
    score2: usize,
    cache: &mut Cache,
) -> (usize, usize) {
    // We swap players on each recursive call, so player 2 is the previous player 1. Player 1 is the
    // only one who played, so we only need to check his score.
    if score2 >= 21 {
        return (0, 1);
    }

    // Memoization
    if let Some(wins) = cache.get(&(pos1, pos2, score1, score2)) {
        return *wins;
    }

    let (mut wins1, mut wins2) = (0, 0);
    // 3 = 1 + 1 + 1
    // 4 = 1 + 1 + 2, 1 + 2 + 1, 2 + 1 + 1
    // ...
    // 9 = 3 + 3 + 3
    for (mv, times) in [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)] {
        let mut pos1 = pos1; // copy
        pos1.advance_by(mv);

        // We swap out player 1 and 2 for the next recursion
        let (w2, w1) = quantum_dice_game(pos2, pos1, score2, score1 + pos1.pos(), cache);

        wins1 += w1 * times;
        wins2 += w2 * times;
    }

    cache.insert((pos1, pos2, score1, score2), (wins1, wins2));
    (wins1, wins2)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

    #[test]
    fn part2_provided() {
        assert_eq!(part2(PROVIDED).unwrap(), 444356092776315);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT).unwrap(), 91559198282731);
    }
}
