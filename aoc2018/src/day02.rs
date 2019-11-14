use std::collections::HashMap;

use super::Result;

const INPUT: &str = include_str!("../input/day02.txt");

pub fn run() -> Result<()> {
    println!("part 1: {}", part1(INPUT)?);

    Ok(())
}

fn part1(input: &str) -> Result<u32> {
    let mut twice = 0;
    let mut thrice = 0;

    for line in input.lines() {
        let mut seen: HashMap<char, u32> = HashMap::new();
        for c in line.chars() {
            *seen.entry(c).or_default() += 1;
        }

        if seen.values().any(|x| *x == 2) {
            twice += 1;
        }

        if seen.values().any(|x| *x == 3) {
            thrice += 1;
        }
    }

    Ok(twice * thrice)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_provided() {
        let input = "abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab
";

        assert_eq!(part1(input).unwrap(), 12);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 5750);
    }
}
