use std::collections::HashSet;
use std::fmt::Write;

const INPUT: &str = include_str!("../input/day06.txt");

pub fn run() -> aoc::Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;
    writeln!(res, "part 2: {}", part2(INPUT)?)?;

    Ok(res)
}

fn get_groups(input: &str) -> aoc::Result<Vec<Group>> {
    let mut groups = Vec::new();

    let mut answers = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            groups.push(Group {
                answers: answers.clone(),
            });
            answers.clear();
        } else {
            let person_answers = line.bytes().collect();
            answers.push(person_answers);
        }
    }

    if !answers.is_empty() {
        groups.push(Group { answers });
    }

    Ok(groups)
}

fn part1(input: &str) -> aoc::Result<usize> {
    let groups = get_groups(input)?;

    Ok(groups
        .iter()
        .map(|group| {
            group.answers.iter().fold(HashSet::new(), |set, answers| {
                set.union(answers).copied().collect()
            })
        })
        .map(|set| set.len())
        .sum())
}

fn part2(input: &str) -> aoc::Result<usize> {
    let groups = get_groups(input)?;

    Ok(groups
        .iter()
        .map(|group| {
            group
                .answers
                .iter()
                .skip(1)
                .fold(group.answers[0].clone(), |set, answers| {
                    set.intersection(answers).copied().collect()
                })
        })
        .map(|set| set.len())
        .sum())
}

struct Group {
    answers: Vec<HashSet<u8>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    static PROVIDED: &'static str = include_str!("../input/day06_provided.txt");

    #[test]
    fn part1_provided() {
        assert_eq!(part1(PROVIDED).unwrap(), 11);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 6382);
    }

    #[test]
    fn part2_provided() {
        assert_eq!(part2(PROVIDED).unwrap(), 6);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT).unwrap(), 3197);
    }
}
