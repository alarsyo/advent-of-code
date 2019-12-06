use std::collections::HashMap;

use aoc::err;
use aoc::Result;

const INPUT: &str = include_str!("../input/day06.txt");

pub fn run() -> Result<()> {
    println!("part 1: {}", part1(INPUT)?);

    Ok(())
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
        .map(|line| line.trim_end())
        .map(|line| {
            let paren = line
                .find(')')
                .ok_or_else(|| err!("couldn't find `)` in line: {}", line))?;
            Ok((line[paren + 1..].to_string(), line[..paren].to_string()))
        })
        .collect::<Result<HashMap<String, String>>>()?;

    let mut cache = HashMap::new();
    Ok(orbits
        .keys()
        .map(|k| count_orbits(&k, &orbits, &mut cache))
        .sum())
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

    #[test]
    fn part1_provided() {
        assert_eq!(part1(PROVIDED1).unwrap(), 42);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 140608);
    }
}
