use std::collections::HashSet;

use super::Result;

const INPUT: &str = include_str!("../input/day05.txt");

pub fn run() -> Result<()> {
    println!("part 1: {}", part1(INPUT));
    println!("part 2: {}", part2(INPUT));

    Ok(())
}

fn same_type(a: char, b: char) -> bool {
    a.to_ascii_lowercase() == b.to_ascii_lowercase()
}

fn remove_type(input: &str, c: char) -> String {
    input.chars().filter(|ch| !same_type(c, *ch)).collect()
}

fn collapse(input: &str) -> String {
    let mut res = String::with_capacity(input.len());

    // tracks last character of string
    let mut last: Option<char> = None;

    for next in input.chars() {
        match last {
            Some(elem) => {
                // if same type but different polarity
                if same_type(elem, next) && elem != next {
                    // drop both elem and next
                    last = res.pop();
                } else {
                    // consider elem "safe" to push
                    res.push(elem);
                    last = Some(next);
                }
            }
            None => {
                last = Some(next);
            }
        }
    }

    // add last character to string if exists
    if let Some(c) = last {
        res.push(c);
    }

    res
}

fn part1(input: &str) -> usize {
    let res = collapse(input);
    res.len()
}

fn part2(input: &str) -> usize {
    let mut set = HashSet::new();

    for c in input.chars() {
        set.insert(c.to_ascii_lowercase());
    }

    set.iter()
        .map(|c| collapse(&remove_type(input, *c)).len())
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED: &str = "dabAcCaCBAcCcaDA";

    #[test]
    fn part1_provided() {
        assert_eq!(part1(PROVIDED), 10);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT), 10638);
    }

    #[test]
    fn part2_provided() {
        assert_eq!(part2(PROVIDED), 4);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT), 4944);
    }
}
