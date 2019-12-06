use std::fmt::Write;

use aoc::err;
use aoc::Result;

const INPUT: &str = include_str!("../input/day04.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;
    writeln!(res, "part 2: {}", part2(INPUT)?)?;

    Ok(res)
}

fn part1(input: &str) -> Result<usize> {
    let mut range = input.trim_end().split('-');
    let begin: usize = range
        .next()
        .ok_or_else(|| err!("invalid input: {}", input))?
        .parse()?;
    let end: usize = range
        .next()
        .ok_or_else(|| err!("invalid input: {}", input))?
        .parse()?;

    let mut digits = Vec::with_capacity(10);
    let mut res = 0;
    for n in begin..=end {
        digits.clear();
        digits.extend(DigitsIter::new(n));

        if digits.windows(2).any(|window| window[0] == window[1])
            && digits.windows(2).all(|window| window[0] <= window[1])
        {
            res += 1;
        }
    }

    Ok(res)
}

fn part2(input: &str) -> Result<usize> {
    let mut range = input.trim_end().split('-');
    let begin: usize = range
        .next()
        .ok_or_else(|| err!("invalid input: {}", input))?
        .parse()?;
    let end: usize = range
        .next()
        .ok_or_else(|| err!("invalid input: {}", input))?
        .parse()?;

    let mut res = 0;
    let mut digits = Vec::with_capacity(10);

    for n in begin..=end {
        digits.clear();
        digits.extend(DigitsIter::new(n));

        let mut ordered = true;
        let mut pair = false;
        let mut count = 1;
        let mut prev = digits[0];
        for i in 1..digits.len() {
            if prev > digits[i] {
                ordered = false;
                break;
            } else if prev == digits[i] {
                count += 1;
            } else {
                if count == 2 {
                    pair = true;
                }
                count = 1;
            }

            prev = digits[i];
        }
        pair = pair || count == 2;

        if pair && ordered {
            res += 1;
        }
    }

    Ok(res)
}

struct DigitsIter {
    n: usize,
    div: usize,
}

impl DigitsIter {
    fn new(n: usize) -> Self {
        let mut div = 1;
        while n >= div * 10 {
            div *= 10;
        }

        DigitsIter { n, div }
    }
}

impl Iterator for DigitsIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        match self.div {
            0 => None,
            _ => {
                let res = self.n / self.div;
                self.n %= self.div;
                self.div /= 10;
                Some(res)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_provided() {
        assert_eq!(part1("111111-111111").unwrap(), 1);
        assert_eq!(part1("223450-223450").unwrap(), 0);
        assert_eq!(part1("123789-123789").unwrap(), 0);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 1729);
    }

    #[test]
    fn part2_provided() {
        assert_eq!(part2("112233-112233").unwrap(), 1);
        assert_eq!(part2("123444-123444").unwrap(), 0);
        assert_eq!(part2("111122-111122").unwrap(), 1);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT).unwrap(), 1172);
    }
}
