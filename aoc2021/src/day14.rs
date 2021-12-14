use std::collections::{HashMap, HashSet, LinkedList};
use std::fmt::Write;

use anyhow::{Context, Result};

const INPUT: &str = include_str!("../input/day14.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;

    Ok(res)
}

fn part1(input: &str) -> Result<usize> {
    let (polymer, rules) = input.split_once("\n\n").context("couldn't split input")?;
    let mut polymer: Polymer = polymer.parse()?;
    let rules: Rules = rules.parse()?;

    let mut molecule_set = polymer.molecule_set();
    rules.0.values().for_each(|v| {
        molecule_set.insert(*v);
    });

    for _ in 0..10 {
        polymer.insert_pairs(&rules.0)?;
    }

    let occurrences = polymer.compute_occurrences(&molecule_set);

    let (_, least_common_occurrences) = occurrences
        .iter()
        .min_by_key(|(_, occurrences)| occurrences)
        .unwrap();
    let (_, most_common_occurrences) = occurrences
        .iter()
        .max_by_key(|(_, occurrences)| occurrences)
        .unwrap();

    Ok(most_common_occurrences - least_common_occurrences)
}

struct Rules(HashMap<(char, char), char>);

impl std::str::FromStr for Rules {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Self(
            s.lines()
                .map(str::trim)
                .map(|l| {
                    let (pair, res) = l.split_once(" -> ").context("couldn't parse rule")?;
                    Ok((
                        (
                            pair.chars().next().context("")?,
                            pair.chars().nth(1).context("")?,
                        ),
                        res.chars().next().context("couldn't parse rule")?,
                    ))
                })
                .collect::<Result<_>>()?,
        ))
    }
}

#[derive(Debug)]
struct Polymer {
    molecules: Option<LinkedList<char>>,
}

impl Polymer {
    fn insert_pairs(&mut self, rules: &HashMap<(char, char), char>) -> Result<()> {
        debug_assert!(self.molecules.is_some());

        self.molecules = Some(insert_pairs(
            std::mem::replace(&mut self.molecules, None).unwrap(),
            rules,
        )?);
        Ok(())
    }

    fn compute_occurrences(&self, molecule_set: &HashSet<char>) -> Vec<(char, usize)> {
        debug_assert!(self.molecules.is_some());

        let mut res = Vec::new();
        for molecule in molecule_set {
            let count = self
                .molecules
                .as_ref()
                .unwrap() // we always have a Some, Option only used for std::mem::replace
                .iter()
                .filter(|&m| m == molecule)
                .count();
            res.push((*molecule, count));
        }

        res
    }

    fn molecule_set(&self) -> HashSet<char> {
        debug_assert!(self.molecules.is_some());

        self.molecules.as_ref().unwrap().iter().copied().collect()
    }
}

impl std::str::FromStr for Polymer {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let molecules = s.trim().chars().collect();

        Ok(Polymer {
            molecules: Some(molecules),
        })
    }
}

fn insert_pairs(
    mut molecules: LinkedList<char>,
    rules: &HashMap<(char, char), char>,
) -> Result<LinkedList<char>> {
    if molecules.len() <= 1 {
        return Ok(molecules);
    }

    // List length is at least 2
    let mut iter = molecules.iter();
    let (left, right) = (*iter.next().unwrap(), *iter.next().unwrap());

    let to_insert = *rules
        .get(&(left, right))
        .with_context(|| format!("couldn't find rule for pair ({}, {})", left, right))?;

    let mut tail = insert_pairs(molecules.split_off(1), rules)?;

    molecules.push_back(to_insert);
    molecules.append(&mut tail);

    Ok(molecules)
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED: &str = include_str!("../input/day14_provided.txt");

    #[test]
    fn part1_provided() {
        assert_eq!(part1(PROVIDED).unwrap(), 1588);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 3247);
    }
}
