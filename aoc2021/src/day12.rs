use std::collections::{HashMap, HashSet};
use std::fmt::Write;

use anyhow::{Context, Result};

const INPUT: &str = include_str!("../input/day12.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;

    Ok(res)
}

fn part1(input: &str) -> Result<usize> {
    let cave_map: CaveMap = input.try_into()?;

    cave_map.count_paths()
}

#[derive(Clone, Copy)]
struct Cave<'a> {
    name: &'a str,
    small: bool,
}

impl<'a> Cave<'a> {
    fn is_end(&self) -> bool {
        self.name == "end"
    }

    fn is_start(&self) -> bool {
        self.name == "start"
    }

    fn is_small(&self) -> bool {
        self.small
    }
}

impl<'a> From<&'a str> for Cave<'a> {
    fn from(s: &'a str) -> Self {
        Cave {
            name: s,
            small: s.chars().all(char::is_lowercase),
        }
    }
}

impl<'a> std::hash::Hash for Cave<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl<'a> std::cmp::PartialEq for Cave<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl<'a> std::cmp::Eq for Cave<'a> {}

struct CaveMap<'a> {
    connections: HashMap<Cave<'a>, Vec<Cave<'a>>>,
}

impl<'a> CaveMap<'a> {
    fn count_paths(&self) -> Result<usize> {
        let start = *self
            .connections
            .keys()
            .find(|cave| cave.is_start())
            .context("couldn't find starting cave")?;
        Ok(self.count_paths_rec(start, HashSet::new()))
    }

    fn count_paths_rec(&self, from: Cave<'a>, mut small_seen: HashSet<Cave<'a>>) -> usize {
        if from.is_end() {
            return 1;
        }

        if from.is_small() {
            if small_seen.contains(&from) {
                return 0;
            }
            small_seen.insert(from);
        }

        let mut paths = 0;
        for dst in &self.connections[&from] {
            paths += self.count_paths_rec(*dst, small_seen.clone());
        }

        paths
    }
}

impl<'a> TryFrom<&'a str> for CaveMap<'a> {
    type Error = anyhow::Error;

    fn try_from(s: &'a str) -> Result<Self> {
        let mut map = HashMap::new();

        for line in s.lines().map(str::trim) {
            let (src, dst) = line
                .split_once('-')
                .context("couldn't parse cave connection")?;
            map.entry(src.into())
                .or_insert_with(Vec::new)
                .push(dst.into());
            map.entry(dst.into())
                .or_insert_with(Vec::new)
                .push(src.into());
        }

        Ok(CaveMap { connections: map })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED1: &str = include_str!("../input/day12_provided1.txt");
    const PROVIDED2: &str = include_str!("../input/day12_provided2.txt");
    const PROVIDED3: &str = include_str!("../input/day12_provided3.txt");

    #[test]
    fn part1_provided() {
        assert_eq!(part1(PROVIDED1).unwrap(), 10);
        assert_eq!(part1(PROVIDED2).unwrap(), 19);
        assert_eq!(part1(PROVIDED3).unwrap(), 226);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 5252);
    }
}
