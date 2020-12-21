use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::fmt::Write;

use anyhow::{Context, Result};

const INPUT: &str = include_str!("../input/day21.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;

    Ok(res)
}

/// Returns all words, and the number of times they appear in the input (useful for part1)
fn get_all_words(input: &str) -> HashMap<&str, usize> {
    input.lines().fold(HashMap::new(), |mut map, line| {
        let left_paren = line.find('(').unwrap();

        line[..(left_paren - 1)].split(' ').for_each(|word| {
            *map.entry(word).or_default() += 1;
        });

        map
    })
}

fn part1(input: &str) -> Result<usize> {
    let matchings: AllergenMatchings = input.try_into()?;
    let all_words = get_all_words(input);

    // identify words that aren't in any allergen possible matching list
    let not_allergens = all_words
        .keys()
        .filter(|&word| !matchings.0.values().any(|set| set.contains(word)));

    Ok(not_allergens.map(|word| all_words[word]).sum())
}

#[derive(Debug)]
struct AllergenMatchings<'a>(HashMap<&'a str, HashSet<&'a str>>);

impl<'a> std::convert::TryFrom<&'a str> for AllergenMatchings<'a> {
    type Error = anyhow::Error;

    fn try_from(s: &'a str) -> Result<Self> {
        let mut matchings = HashMap::new();

        for line in s.lines() {
            let left_paren = line.find('(').context("couldn't find open paren in line")?;
            let right_paren = line
                .find(')')
                .context("couldn't find closing paren in line")?;

            let allergens = line[(left_paren + "contains ".len() + 1)..right_paren].split(", ");

            let words: HashSet<&str> = line[..(left_paren - 1)].split(' ').collect();

            // update potential matches for each allergen mentioned in line
            for allergen in allergens {
                let set = matchings.entry(allergen).or_insert_with(|| words.clone());
                *set = set.intersection(&words).copied().collect();
            }
        }

        Ok(AllergenMatchings(matchings))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED: &str = include_str!("../input/day21_provided.txt");

    #[test]
    fn part1_provided() {
        assert_eq!(part1(PROVIDED).unwrap(), 5);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 2315);
    }
}
