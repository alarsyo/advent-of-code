use std::fmt::Write;

use anyhow::Result;

const INPUT: &str = include_str!("../input/day01.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;
    writeln!(res, "part 2: {}", part2(INPUT)?)?;

    Ok(res)
}

fn part1(input: &str) -> Result<u64> {
    let digit_list = input
        .lines()
        .map(|s| s.chars().filter(char::is_ascii_digit).collect::<String>());

    Ok(digit_list
        .map(|digits| {
            let first = digits.chars().next().unwrap().to_digit(10).unwrap() as u64;
            let last = digits.chars().last().unwrap().to_digit(10).unwrap() as u64;

            first * 10 + last
        })
        .sum())
}

fn part2(_input: &str) -> Result<u64> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED: &str = include_str!("../input/day01_provided.txt");

    #[test]
    fn part1_provided() {
        assert_eq!(part1(PROVIDED).unwrap(), 142);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 54697);
    }
}
