use aoc::Result;

const INPUT: &str = include_str!("../input/day05.txt");

pub fn run() -> Result<()> {
    println!("part 1: {}", part1(INPUT));
    println!("part 2: {}", part2(INPUT));

    Ok(())
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.trim_end())
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
        .filter(|line| line.find("ab").is_none())
        .filter(|line| line.find("cd").is_none())
        .filter(|line| line.find("pq").is_none())
        .filter(|line| line.find("xy").is_none())
        .count()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.trim_end())
        .filter(|line| {
            for i in 0..(line.chars().count() - 3) {
                let seq = &line[i..(i + 2)];
                let line = &line[(i + 2)..];
                if line.find(seq).is_some() {
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
