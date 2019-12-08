use std::fmt::Write;

use aoc::err;
use aoc::Result;

const INPUT: &str = include_str!("../input/day04.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    let (begin, end) = range(INPUT)?;

    writeln!(res, "part 1: {}", part1(begin, end)?)?;
    writeln!(res, "part 2: {}", part2(begin, end)?)?;

    Ok(res)
}

fn range(input: &str) -> Result<(usize, usize)> {
    let mut range = input.trim_end().split('-');
    let begin = range
        .next()
        .ok_or_else(|| err!("invalid input: {}", input))?
        .parse()?;
    let end = range
        .next()
        .ok_or_else(|| err!("invalid input: {}", input))?
        .parse()?;

    Ok((begin, end))
}

fn part1(begin: usize, end: usize) -> Result<usize> {
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

fn part2(begin: usize, end: usize) -> Result<usize> {
    let mut res = 0;
    let mut digits = Vec::with_capacity(10);

    for n in begin..=end {
        digits.clear();
        digits.extend(DigitsIter::new(n));

        let mut ordered = true;
        let mut pair = false;
        let mut count = 1;
        let mut prev = digits[0];
        for digit in digits.iter().skip(1) {
            if prev > *digit {
                ordered = false;
                break;
            } else if prev == *digit {
                count += 1;
            } else {
                if count == 2 {
                    pair = true;
                }
                count = 1;
            }

            prev = *digit;
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
        let (begin, end) = range("111111-111111").unwrap();
        println!("{}, {}", begin, end);
        assert_eq!(part1(begin, end).unwrap(), 1);
        let (begin, end) = range("223450-223450").unwrap();
        assert_eq!(part1(begin, end).unwrap(), 0);
        let (begin, end) = range("123789-123789").unwrap();
        assert_eq!(part1(begin, end).unwrap(), 0);
    }

    #[test]
    fn part1_real() {
        let (begin, end) = range(INPUT).unwrap();
        assert_eq!(part1(begin, end).unwrap(), 1729);
    }

    #[test]
    fn part2_provided() {
        let (begin, end) = range("112233-112233").unwrap();
        assert_eq!(part2(begin, end).unwrap(), 1);
        let (begin, end) = range("123444-123444").unwrap();
        assert_eq!(part2(begin, end).unwrap(), 0);
        let (begin, end) = range("111122-111122").unwrap();
        assert_eq!(part2(begin, end).unwrap(), 1);
    }

    #[test]
    fn part2_real() {
        let (begin, end) = range(INPUT).unwrap();
        assert_eq!(part2(begin, end).unwrap(), 1172);
    }
}
