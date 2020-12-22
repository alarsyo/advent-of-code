use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fmt::Write;

use anyhow::{Context, Result};

const INPUT: &str = include_str!("../input/day22.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;

    Ok(res)
}

fn play_game<'a>(deck_a: &'a mut Deck, deck_b: &'a mut Deck) -> &'a Deck {
    while !(deck_a.0.is_empty() || deck_b.0.is_empty()) {
        let card_a = deck_a.0.pop_front().unwrap();
        let card_b = deck_b.0.pop_front().unwrap();

        match card_a.cmp(&card_b) {
            Ordering::Greater => {
                deck_a.0.push_back(card_a);
                deck_a.0.push_back(card_b);
            }
            Ordering::Less => {
                deck_b.0.push_back(card_b);
                deck_b.0.push_back(card_a);
            }
            Ordering::Equal => unreachable!(),
        }
    }

    if deck_a.0.is_empty() {
        deck_b
    } else {
        deck_a
    }
}

fn part1(input: &str) -> Result<u64> {
    let mut decks = input.split("\n\n");

    let mut deck_a: Deck = decks.next().context("couldn't get first deck")?.parse()?;
    let mut deck_b: Deck = decks.next().context("couldn't get second deck")?.parse()?;

    let winning_deck = play_game(&mut deck_a, &mut deck_b);

    Ok(winning_deck
        .0
        .iter()
        .rev()
        .enumerate()
        .map(|(i, card)| card * (i as u64 + 1))
        .sum())
}

struct Deck(VecDeque<u64>);

impl Deck {}

impl std::str::FromStr for Deck {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut lines = s.lines();

        // `Player N:`
        lines.next().context("couldn't skip first line")?;

        let deck = lines
            .map(|line| line.parse().map_err(anyhow::Error::new))
            .collect::<Result<_>>()?;

        Ok(Self(deck))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED: &str = include_str!("../input/day22_provided.txt");

    #[test]
    fn part1_provided() {
        assert_eq!(part1(PROVIDED).unwrap(), 306);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 30780);
    }
}
