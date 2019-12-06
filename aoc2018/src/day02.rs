use std::collections::HashMap;
use std::fmt::Write;

use aoc::Result;

const INPUT: &str = include_str!("../input/day02.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;
    writeln!(res, "part 2: {}", part2(INPUT)?)?;

    Ok(res)
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

fn part2(input: &str) -> Result<String> {
    let lines = input.lines().collect::<Vec<&str>>();

    for i in 0..lines.len() {
        for j in (i + 1)..lines.len() {
            let different = lines[i]
                .chars()
                .zip(lines[j].chars())
                .filter(|tuple| tuple.0 != tuple.1)
                .count();

            if different == 1 {
                return Ok(common_letters(lines[i], lines[j]));
            }
        }
    }

    Ok("".into())
}

fn common_letters(a: &str, b: &str) -> String {
    a.chars()
        .zip(b.chars())
        .filter_map(|e| match e {
            (a, b) if a == b => Some(a),
            _ => None,
        })
        .collect()
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

    #[test]
    fn part2_provided() {
        let input = "abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz
";

        assert_eq!(part2(input).unwrap(), "fgij");
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT).unwrap(), "tzyvunogzariwkpcbdewmjhxi");
    }
}
