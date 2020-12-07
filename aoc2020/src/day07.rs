use std::collections::HashMap;
use std::fmt::Write;
use std::str::FromStr;

use aoc::err;

const INPUT: &str = include_str!("../input/day07.txt");

pub fn run() -> aoc::Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;
    writeln!(res, "part 2: {}", part2(INPUT)?)?;

    Ok(res)
}

fn part1(input: &str) -> aoc::Result<usize> {
    let bag_rules = input
        .lines()
        .map(|line| line.parse())
        .collect::<aoc::Result<Vec<BagRule>>>()
        .unwrap();

    // create map with Key = color, Value = BagRule
    let bag_rules_map = bag_rules
        .iter()
        .map(|bag_rule| (bag_rule.color.clone(), bag_rule.clone()))
        .collect();

    Ok(bag_rules
        .iter()
        .filter(|bag| bag.can_contain("shiny gold", &bag_rules_map))
        .count())
}

fn part2(input: &str) -> aoc::Result<usize> {
    let bag_rules = input
        .lines()
        .map(|line| line.parse())
        .collect::<aoc::Result<Vec<BagRule>>>()
        .unwrap();

    // create map with Key = color, Value = BagRule
    let bag_rules_map: HashMap<String, BagRule> = bag_rules
        .iter()
        .map(|bag_rule| (bag_rule.color.clone(), bag_rule.clone()))
        .collect();

    let shiny_gold = &bag_rules_map["shiny gold"];

    Ok(shiny_gold.num_inner_bags(&bag_rules_map))
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct BagRule {
    color: String,
    contains: Vec<(usize, String)>,
}

impl BagRule {
    fn can_contain(&self, color: &str, all_bags: &HashMap<String, BagRule>) -> bool {
        if self.contains.iter().any(|(_, c)| c == color) {
            return true;
        }

        self.contains.iter().any(|(_, c)| {
            // fetch rules for this bag in map
            let bag_rule = &all_bags[c];

            bag_rule.can_contain(color, all_bags)
        })
    }

    fn num_inner_bags(&self, all_bags: &HashMap<String, BagRule>) -> usize {
        self.contains
            .iter()
            .map(|(count, c)| {
                // fetch rules for this bag in map
                let bag_rule = &all_bags[c];

                count + count * bag_rule.num_inner_bags(all_bags)
            })
            .sum()
    }
}

impl FromStr for BagRule {
    type Err = aoc::Error;

    fn from_str(s: &str) -> aoc::Result<Self> {
        let words: Vec<&str> = s.split(' ').collect();

        // let's assume our input is always valid for now
        let adjective = words[0];
        let color = words[1];

        debug_assert!(words[2] == "bags");
        debug_assert!(words[3] == "contain");

        let mut words = &words[4..];
        let mut contains = Vec::new();

        loop {
            match words[0] {
                // this bag doesn't contain any other bag, we can stop
                "no" => {
                    debug_assert!(words[1] == "other");
                    debug_assert!(words[2] == "bags.");
                    break;
                }
                // this is a list of bags that should be contained, parse the first one then loop
                // again
                number => {
                    let n = number
                        .parse()
                        .map_err(|e| err!("couldn't parse number `{}` in bag rule", e))?;

                    let adjective = words[1];
                    let color = words[2];

                    contains.push((n, format!("{} {}", adjective, color)));

                    match words[3] {
                        // there are other bags in this one
                        "bag," | "bags," => {
                            words = &words[4..];
                        }
                        // this was the last bag
                        "bag." | "bags." => break,
                        _ => todo!("handle this with error"),
                    }
                }
            }
        }

        Ok(Self {
            color: format!("{} {}", adjective, color),
            contains,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static PROVIDED1: &'static str = include_str!("../input/day07_provided1.txt");
    static PROVIDED2: &'static str = include_str!("../input/day07_provided2.txt");

    #[test]
    fn part1_provided_parse() {
        let bag_rules = PROVIDED1
            .lines()
            .map(|line| line.parse())
            .collect::<aoc::Result<Vec<BagRule>>>()
            .unwrap();

        let expected = vec![
            BagRule {
                color: "light red".to_string(),
                contains: vec![
                    (1, "bright white".to_string()),
                    (2, "muted yellow".to_string()),
                ],
            },
            BagRule {
                color: "dark orange".to_string(),
                contains: vec![
                    (3, "bright white".to_string()),
                    (4, "muted yellow".to_string()),
                ],
            },
            BagRule {
                color: "bright white".to_string(),
                contains: vec![(1, "shiny gold".to_string())],
            },
            BagRule {
                color: "muted yellow".to_string(),
                contains: vec![(2, "shiny gold".to_string()), (9, "faded blue".to_string())],
            },
            BagRule {
                color: "shiny gold".to_string(),
                contains: vec![
                    (1, "dark olive".to_string()),
                    (2, "vibrant plum".to_string()),
                ],
            },
            BagRule {
                color: "dark olive".to_string(),
                contains: vec![
                    (3, "faded blue".to_string()),
                    (4, "dotted black".to_string()),
                ],
            },
            BagRule {
                color: "vibrant plum".to_string(),
                contains: vec![
                    (5, "faded blue".to_string()),
                    (6, "dotted black".to_string()),
                ],
            },
            BagRule {
                color: "faded blue".to_string(),
                contains: vec![],
            },
            BagRule {
                color: "dotted black".to_string(),
                contains: vec![],
            },
        ];

        assert_eq!(bag_rules.len(), expected.len());

        for (parsed, expected) in expected.into_iter().zip(bag_rules) {
            assert_eq!(parsed, expected);
        }
    }

    #[test]
    fn part1_provided_compute() {
        assert_eq!(part1(PROVIDED1).unwrap(), 4);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 272);
    }

    #[test]
    fn part2_provided() {
        assert_eq!(part2(PROVIDED1).unwrap(), 32);
        assert_eq!(part2(PROVIDED2).unwrap(), 126);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT).unwrap(), 172246);
    }
}
