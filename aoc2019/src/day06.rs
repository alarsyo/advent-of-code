use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Write;
use std::iter;

use anyhow::{Context, Result};

const INPUT: &str = include_str!("../input/day06.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;
    writeln!(res, "part 2: {}", part2(INPUT)?)?;

    Ok(res)
}

fn count_orbits(
    key: &str,
    orbits: &HashMap<String, String>,
    cache: &mut HashMap<String, u64>,
) -> u64 {
    match cache.get(key) {
        Some(val) => *val,
        None => {
            let val = match orbits.get(key) {
                Some(parent) => count_orbits(parent, orbits, cache) + 1,
                None => 0,
            };
            cache.insert(key.to_string(), val);
            val
        }
    }
}

fn part1(input: &str) -> Result<u64> {
    let orbits = input
        .lines()
        .map(str::trim_end)
        .map(|line| {
            let paren = line
                .find(')')
                .with_context(|| format!("couldn't find `)` in line: {}", line))?;
            Ok((line[paren + 1..].to_string(), line[..paren].to_string()))
        })
        .collect::<Result<HashMap<String, String>>>()?;

    let mut cache = HashMap::new();
    Ok(orbits
        .keys()
        .map(|k| count_orbits(k, &orbits, &mut cache))
        .sum())
}

fn part2(input: &str) -> Result<usize> {
    let orbits = input
        .lines()
        .map(str::trim_end)
        .map(|line| {
            let paren = line
                .find(')')
                .with_context(|| format!("couldn't find `)` in line: {}", line))?;
            Ok((line[paren + 1..].to_string(), line[..paren].to_string()))
        })
        .collect::<Result<HashMap<String, String>>>()?;

    let succ = |key: &String| orbits.get(key).cloned();

    let you_path = iter::successors(Some("YOU".to_string()), succ).collect::<HashSet<_>>();
    let santa_path = iter::successors(Some("SAN".to_string()), succ).collect::<HashSet<_>>();

    Ok(you_path.symmetric_difference(&santa_path).count() - 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED1: &str = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
";

    const PROVIDED2: &str = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN
";

    #[test]
    fn part1_provided() {
        assert_eq!(part1(PROVIDED1).unwrap(), 42);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 140608);
    }

    #[test]
    fn part2_provided() {
        assert_eq!(part2(PROVIDED2).unwrap(), 4);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT).unwrap(), 337);
    }
}
