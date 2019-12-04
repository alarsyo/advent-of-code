use aoc::err;
use aoc::Result;

const INPUT: &str = include_str!("../input/day04.txt");

pub fn run() -> Result<()> {
    println!("part 1: {}", part1(INPUT)?);

    Ok(())
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

    let res = (begin..=end)
        .map(|n| DigitsIter::new(n).collect::<Vec<_>>())
        .filter(|digits| digits.windows(2).any(|window| window[0] == window[1]))
        .filter(|digits| digits.windows(2).all(|window| window[0] <= window[1]))
        .count();

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
}
