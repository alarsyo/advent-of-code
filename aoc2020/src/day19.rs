use std::collections::HashMap;
use std::fmt::Write;

use anyhow::{Context, Result};

const INPUT: &str = include_str!("../input/day19.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;
    writeln!(res, "part 2: {}", part2(INPUT)?)?;

    Ok(res)
}

fn get_rules(input: &str) -> Result<HashMap<usize, Rule>> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");

            let idx = parts
                .next()
                .context("no idx on left side of colon for rule")?
                .parse()?;

            let rule = parts
                .next()
                .context("no rule on right side of colon for rule")?
                .parse()?;

            Ok((idx, rule))
        })
        .collect()
}

fn part1(input: &str) -> Result<usize> {
    let mut parts = input.split("\n\n");

    let rules = parts.next().context("no rules before linebreak")?;
    let rules = get_rules(rules)?;

    let lines = parts.next().context("no lines after linebreak")?.lines();

    Ok(lines.filter(|l| rules[&0].matches(&rules, l)).count())
}

fn part2(input: &str) -> Result<usize> {
    let mut parts = input.split("\n\n");

    let rules = parts.next().context("no rules before linebreak")?;
    let mut rules = get_rules(rules)?;

    let lines = parts.next().context("no lines after linebreak")?.lines();

    rules.insert(
        8,
        Rule::Either(
            Rule::Chain(vec![42]).into(),
            Rule::Chain(vec![42, 8]).into(),
        ),
    );
    rules.insert(
        11,
        Rule::Either(
            Rule::Chain(vec![42, 31]).into(),
            Rule::Chain(vec![42, 11, 31]).into(),
        ),
    );

    Ok(lines.filter(|l| rules[&0].matches(&rules, l)).count())
}

#[derive(Debug)]
enum Rule {
    Character(char),
    Chain(Vec<usize>),
    Either(Box<Rule>, Box<Rule>),
}

impl Rule {
    fn matches_rec<'a>(&self, rules: &HashMap<usize, Rule>, s: &'a str) -> Vec<&'a str> {
        if s.is_empty() {
            return vec![];
        }

        match self {
            Rule::Character(c) => {
                let mut res = Vec::new();
                if s.chars().next().unwrap() == *c {
                    res.push(&s[1..]);
                }
                res
            }

            Rule::Chain(idxs) => {
                let mut partial_matches = vec![s];

                for idx in idxs {
                    let rule = &rules[idx];
                    let mut new_partial_matches = Vec::new();
                    for partial_match in partial_matches {
                        new_partial_matches.append(&mut rule.matches_rec(rules, partial_match));
                    }
                    partial_matches = new_partial_matches;
                }

                partial_matches
            }
            Rule::Either(r1, r2) => {
                let mut res = r1.matches_rec(rules, s);
                res.append(&mut r2.matches_rec(rules, s));

                res
            }
        }
    }

    fn matches(&self, rules: &HashMap<usize, Rule>, s: &str) -> bool {
        let res = self.matches_rec(rules, s);

        res.contains(&"")
    }
}

impl std::str::FromStr for Rule {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        if let Some(idx) = s.find('"') {
            let c = s[(idx + 1)..]
                .chars()
                .next()
                .context("couldn't find char after double quote in rule")?;
            return Ok(Rule::Character(c));
        }

        if s.find('|').is_some() {
            let mut rules = s.split(" | ");

            let rule1 = rules.next().context("expected rule on left side of `|`")?;
            let rule2 = rules.next().context("expected rule on right side of `|`")?;

            let rule1 = rule1.parse()?;
            let rule2 = rule2.parse()?;

            return Ok(Rule::Either(Box::new(rule1), Box::new(rule2)));
        }

        let nums = s
            .split(' ')
            .map(|n| n.parse().map_err(anyhow::Error::new))
            .collect::<Result<_>>()?;

        Ok(Rule::Chain(nums))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED1: &str = include_str!("../input/day19_provided1.txt");
    const PROVIDED2: &str = include_str!("../input/day19_provided2.txt");

    #[test]
    fn part1_provided() {
        let mut parts = PROVIDED1.split("\n\n");

        let rules = get_rules(parts.next().unwrap()).unwrap();

        let tests = &[
            ("ababbb", true),
            ("bababa", false),
            ("abbbab", true),
            ("aaabbb", false),
            ("aaaabbb", false),
        ];

        for (test, expected) in tests {
            assert_eq!(
                rules[&0].matches(&rules, test),
                *expected,
                "input: `{}`",
                test
            );
        }
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 144);
    }

    #[test]
    fn part2_provided() {
        assert_eq!(part2(PROVIDED2).unwrap(), 12);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT).unwrap(), 260);
    }
}
