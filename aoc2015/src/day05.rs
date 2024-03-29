use std::fmt::Write;

use anyhow::Result;

const INPUT: &str = include_str!("../input/day05.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT))?;
    writeln!(res, "part 2: {}", part2(INPUT))?;

    Ok(res)
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(str::trim_end)
        .filter(|line| {
            let mut vowel_count = 0;
            for c in line.chars() {
                if "aeiou".find(c).is_some() {
                    vowel_count += 1;
                }
            }

            vowel_count >= 3
        })
        .filter(|line| {
            let mut prev = None;
            for c in line.chars() {
                if let Some(p) = prev {
                    if p == c {
                        return true;
                    }
                }

                prev = Some(c);
            }

            false
        })
        .filter(|line| !line.contains("ab"))
        .filter(|line| !line.contains("cd"))
        .filter(|line| !line.contains("pq"))
        .filter(|line| !line.contains("xy"))
        .count()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(str::trim_end)
        .filter(|line| {
            for i in 0..(line.chars().count() - 3) {
                let seq = &line[i..(i + 2)];
                let line = &line[(i + 2)..];
                if line.contains(seq) {
                    return true;
                }
            }

            false
        })
        .filter(|line| {
            for i in 0..(line.chars().count() - 2) {
                if line.chars().nth(i).unwrap() == line.chars().nth(i + 2).unwrap() {
                    return true;
                }
            }

            false
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_provided() {
        assert_eq!(part1("ugknbfddgicrmopn"), 1);
        assert_eq!(part1("aaa"), 1);
        assert_eq!(part1("jchzalrnumimnmhp"), 0);
        assert_eq!(part1("haegwjzuvuyypxyu"), 0);
        assert_eq!(part1("dvszwmarrgswjxmb"), 0);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT), 258);
    }

    #[test]
    fn part2_provided() {
        assert_eq!(part2("qjhvhtzxzqqjkmpb"), 1);
        assert_eq!(part2("xxyxx"), 1);
        assert_eq!(part2("uurcxstgmygtbstg"), 0);
        assert_eq!(part2("ieodomkazucvgmuy"), 0);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT), 53);
    }
}
