use std::collections::HashMap;
use std::fmt::Write;

use anyhow::{Context, Result};

const INPUT: &str = include_str!("../input/day14.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;
    writeln!(res, "part 2: {}", part2(INPUT)?)?;

    Ok(res)
}

fn part1(input: &str) -> Result<usize> {
    let (polymer, rules) = input.split_once("\n\n").context("couldn't split input")?;
    let mut polymer: Polymer = polymer.parse()?;
    let rules: Rules = rules.parse()?;

    for _ in 0..10 {
        polymer.insert_pairs(&rules.0)?;
    }

    let occurrences = polymer.compute_occurrences();

    let (_, least_common_occurrences) = occurrences
        .iter()
        .min_by_key(|&(_, occurrences)| occurrences)
        .unwrap();
    let (_, most_common_occurrences) = occurrences
        .iter()
        .max_by_key(|&(_, occurrences)| occurrences)
        .unwrap();

    Ok(most_common_occurrences - least_common_occurrences)
}

fn part2(input: &str) -> Result<usize> {
    let (polymer, rules) = input.split_once("\n\n").context("couldn't split input")?;
    let mut polymer: Polymer = polymer.parse()?;
    let rules: Rules = rules.parse()?;

    for _ in 0..40 {
        polymer.insert_pairs(&rules.0)?;
    }

    let occurrences = polymer.compute_occurrences();

    let (_, least_common_occurrences) = occurrences
        .iter()
        .min_by_key(|&(_, occurrences)| occurrences)
        .unwrap();
    let (_, most_common_occurrences) = occurrences
        .iter()
        .max_by_key(|&(_, occurrences)| occurrences)
        .unwrap();

    Ok(most_common_occurrences - least_common_occurrences)
}

struct Rules(HashMap<(u8, u8), u8>);

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
                            *pair.as_bytes().get(0).context("couldn't parse rule")?,
                            *pair.as_bytes().get(1).context("couldn't parse rule")?,
                        ),
                        res.bytes().next().context("couldn't parse rule")?,
                    ))
                })
                .collect::<Result<_>>()?,
        ))
    }
}

#[derive(Debug)]
struct Polymer {
    molecules: HashMap<(u8, u8), usize>,
    first: u8,
    last: u8,
}

impl Polymer {
    fn insert_pairs(&mut self, rules: &HashMap<(u8, u8), u8>) -> Result<()> {
        let mut new_molecules = HashMap::new();

        for (&(a, b), &count) in &self.molecules {
            let middle = *rules
                .get(&(a, b))
                .with_context(|| format!("couldn't find rule for pair ({}, {})", a, b))?;

            *new_molecules.entry((a, middle)).or_insert(0) += count;
            *new_molecules.entry((middle, b)).or_insert(0) += count;
        }

        self.molecules = new_molecules;

        Ok(())
    }

    fn compute_occurrences(&self) -> Vec<(u8, usize)> {
        let mut counts = HashMap::new();

        for (&(a, b), &count) in &self.molecules {
            *counts.entry(a).or_insert(0) += count;
            *counts.entry(b).or_insert(0) += count;
        }

        // the first and last molecule are only counted once, all other molecules are counted twice
        *counts.entry(self.first).or_insert(0) += 1;
        *counts.entry(self.last).or_insert(0) += 1;

        counts
            .into_iter()
            .map(|(m, count)| (m, count / 2))
            .collect()
    }
}

impl std::str::FromStr for Polymer {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut molecules = HashMap::new();
        s.as_bytes()
            .windows(2)
            .for_each(|w| *molecules.entry((w[0], w[1])).or_insert(0) += 1);

        Ok(Polymer {
            molecules,
            first: *s.as_bytes().first().context("polymer was empty")?,
            last: *s.as_bytes().last().context("polymer was empty")?,
        })
    }
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

    #[test]
    fn part2_provided() {
        assert_eq!(part2(PROVIDED).unwrap(), 2188189693529);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT).unwrap(), 4110568157153);
    }
}
