use std::fmt::Write;

use anyhow::Result;

const INPUT: &str = include_str!("../input/day01.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;
    writeln!(res, "part 2: {}", part2(INPUT)?)?;

    Ok(res)
}

fn part1(input: &str) -> Result<usize> {
    let measurements = input
        .lines()
        .map(|line| line.parse::<u64>().map_err(anyhow::Error::new))
        .collect::<Result<Vec<_>>>()?;

    Ok(measurements.windows(2).filter(|w| w[0] < w[1]).count())
}

fn part2(input: &str) -> Result<usize> {
    let measurements = input
        .lines()
        .map(|line| line.parse::<u64>().map_err(anyhow::Error::new))
        .collect::<Result<Vec<_>>>()?;

    let mut increases = 0;
    let mut prev: Option<u64> = None;

    for window in measurements.windows(3) {
        let sum = window.iter().sum();
        if let Some(prev) = prev {
            if prev < sum {
                increases += 1;
            }
        }
        prev = Some(sum);
    }

    Ok(increases)
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED: &str = include_str!("../input/day01_provided.txt");

    #[test]
    fn part1_provided() {
        assert_eq!(part1(PROVIDED).unwrap(), 7);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 1502);
    }

    #[test]
    fn part2_provided() {
        assert_eq!(part2(PROVIDED).unwrap(), 5);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT).unwrap(), 1538);
    }
}
