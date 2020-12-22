use std::cmp::Ordering;
use std::collections::{HashSet, VecDeque};
use std::fmt::Write;

use anyhow::{Context, Result};

const INPUT: &str = include_str!("../input/day22.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;
    writeln!(res, "part 2: {}", part2(INPUT)?)?;

    Ok(res)
}

fn play_game(mut deck_a: Deck, mut deck_b: Deck) -> Deck {
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

fn play_recursive_game(mut deck_a: Deck, mut deck_b: Deck) -> (Deck, bool) {
    let mut seen: HashSet<(Deck, Deck)> = HashSet::new();

    while !(deck_a.0.is_empty() || deck_b.0.is_empty()) {
        // Before either player deals a card, if there was a previous round in this game that had
        // exactly the same cards in the same order in the same players' decks, the game instantly
        // ends in a win for player 1. Previous rounds from other games are not considered. (This
        // prevents infinite games of Recursive Combat, which everyone agrees is a bad idea.)
        if seen.contains(&(deck_a.clone(), deck_b.clone())) {
            return (deck_a, true);
        } else {
            seen.insert((deck_a.clone(), deck_b.clone()));
        }

        // Otherwise, this round's cards must be in a new configuration; the players begin the round
        // by each drawing the top card of their deck as normal.
        let card_a = deck_a.0.pop_front().unwrap();
        let card_b = deck_b.0.pop_front().unwrap();

        // true if first player won, false otherwise
        let winner: bool;

        if deck_a.0.len() >= card_a as usize && deck_b.0.len() >= card_b as usize {
            // If both players have at least as many cards remaining in their deck as the value of
            // the card they just drew, the winner of the round is determined by playing a new game
            // of Recursive Combat (see below).

            // To play a sub-game of Recursive Combat, each player creates a new deck by making a
            // copy of the next cards in their deck (the quantity of cards copied is equal to the
            // number on the card they drew to trigger the sub-game). During this sub-game, the game
            // that triggered it is on hold and completely unaffected; no cards are removed from
            // players' decks to form the sub-game. (For example, if player 1 drew the 3 card, their
            // deck in the sub-game would be copies of the next three cards in their deck.)
            let mut new_deck_a = deck_a.clone();
            let mut new_deck_b = deck_b.clone();
            new_deck_a.0.truncate(card_a as usize);
            new_deck_b.0.truncate(card_b as usize);

            let (_, deck_a_won) = play_recursive_game(new_deck_a, new_deck_b);
            winner = deck_a_won;
        } else {
            // Otherwise, at least one player must not have enough cards left in their deck to
            // recurse; the winner of the round is the player with the higher-value card.
            match card_a.cmp(&card_b) {
                Ordering::Greater => winner = true,
                Ordering::Less => winner = false,
                Ordering::Equal => unreachable!(),
            }
        }

        if winner {
            deck_a.0.push_back(card_a);
            deck_a.0.push_back(card_b);
        } else {
            deck_b.0.push_back(card_b);
            deck_b.0.push_back(card_a);
        }
    }

    if deck_a.0.is_empty() {
        (deck_b, false)
    } else {
        (deck_a, true)
    }
}

fn deck_score(deck: &Deck) -> u64 {
    deck.0
        .iter()
        .rev()
        .enumerate()
        .map(|(i, card)| card * (i as u64 + 1))
        .sum()
}

fn part1(input: &str) -> Result<u64> {
    let mut decks = input.split("\n\n");

    let deck_a: Deck = decks.next().context("couldn't get first deck")?.parse()?;
    let deck_b: Deck = decks.next().context("couldn't get second deck")?.parse()?;

    let winning_deck = play_game(deck_a, deck_b);

    Ok(deck_score(&winning_deck))
}

fn part2(input: &str) -> Result<u64> {
    let mut decks = input.split("\n\n");

    let deck_a: Deck = decks.next().context("couldn't get first deck")?.parse()?;
    let deck_b: Deck = decks.next().context("couldn't get second deck")?.parse()?;

    let (winning_deck, _) = play_recursive_game(deck_a, deck_b);

    Ok(deck_score(&winning_deck))
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
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

    #[test]
    fn part2_provided() {
        assert_eq!(part2(PROVIDED).unwrap(), 291);
    }

    #[test]
    #[ignore]
    fn part2_real() {
        assert_eq!(part2(INPUT).unwrap(), 36621);
    }
}
