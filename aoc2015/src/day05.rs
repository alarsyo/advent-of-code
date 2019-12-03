use aoc::Result;

const INPUT: &str = include_str!("../input/day05.txt");

pub fn run() -> Result<()> {
    println!("part 1: {}", part1(INPUT));

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
}
