use std::{fmt::Write, str::FromStr};

use anyhow::{Context, Result, bail};

const INPUT: &str = include_str!("../input/day06.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);
    writeln!(res, "part 1: {}", part1(INPUT)?)?;
    writeln!(res, "part 2: {}", part2(INPUT)?)?;
    Ok(res)
}

#[derive(Debug, Clone, Copy)]
enum MathOp {
    Add,
    Mult,
}

impl FromStr for MathOp {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "+" => Ok(MathOp::Add),
            "*" => Ok(MathOp::Mult),
            _ => bail!("cannot parse `{}' as MathOp", s),
        }
    }
}

#[derive(Debug, Clone)]
struct MathProblem {
    numbers: Vec<u64>,
    op: MathOp,
}

impl MathProblem {
    fn compute(&self) -> u64 {
        match self.op {
            MathOp::Add => self.numbers.iter().sum(),
            MathOp::Mult => self.numbers.iter().product(),
        }
    }
}

fn part1(input: &str) -> Result<u64> {
    let mut numbers: Vec<Vec<u64>> = Vec::new();
    let mut ops = Vec::new();

    for line in input.lines() {
        let line = line.trim();
        if line
            .chars()
            .next()
            .context("got an empty line")?
            .is_ascii_punctuation()
        {
            ops = line
                .split_ascii_whitespace()
                .map(|c| c.parse::<MathOp>())
                .collect::<Result<_>>()?;
            break;
        }

        let split_line = line.trim().split_ascii_whitespace();
        let width = split_line.clone().count();
        if !numbers.is_empty() && numbers.len() != width {
            bail!(
                "all input lines should have the same number of columns: current line has {}, previous lines had {}",
                width,
                numbers.len()
            );
        }
        if numbers.is_empty() {
            for _ in 0..width {
                numbers.push(Vec::new());
            }
        }

        for (idx, num) in split_line.enumerate() {
            numbers[idx].push(num.parse::<u64>()?);
        }
    }

    Ok(ops
        .into_iter()
        .zip(numbers)
        .map(|(op, numbers)| MathProblem { numbers, op })
        .map(|problem| problem.compute())
        .sum())
}

fn part2(input: &str) -> Result<u64> {
    let mut numbers: Vec<Vec<u8>> = Vec::new();
    let mut ops = Vec::new();

    for line in input.lines() {
        if line
            .chars()
            .next()
            .context("got an empty line")?
            .is_ascii_punctuation()
        {
            ops = line
                .split_ascii_whitespace()
                .map(|c| c.parse::<MathOp>())
                .collect::<Result<_>>()?;
            break;
        }

        let line = line.as_bytes().to_vec();
        if !numbers.is_empty() && numbers[0].len() != line.len() {
            bail!(
                "all input lines should have the same number of columns: current line has {}, previous lines had {}",
                line.len(),
                numbers[0].len(),
            );
        }
        numbers.push(line);
    }

    if numbers.is_empty() {
        bail!("input had no numbers");
    }

    let mut parsed_numbers = Vec::new();
    let mut cur_problem_numbers = Vec::new();
    for col in 0..numbers[0].len() {
        fn col_is_empty(col: usize, numbers: &[Vec<u8>]) -> bool {
            (0..numbers.len()).all(|line| numbers[line][col].is_ascii_whitespace())
        }

        if col_is_empty(col, &numbers) {
            parsed_numbers.push(cur_problem_numbers);
            cur_problem_numbers = Vec::new();
            continue;
        }

        let mut num = 0;
        for line in &numbers {
            let chr = line[col];
            if chr.is_ascii_whitespace() {
                continue;
            }
            if !chr.is_ascii_digit() {
                bail!("input contained non-digit ascii character: `{}'", chr)
            }
            let digit = (chr - b'0') as u64;
            num *= 10;
            num += digit;
        }
        cur_problem_numbers.push(num);
    }
    if !cur_problem_numbers.is_empty() {
        parsed_numbers.push(cur_problem_numbers);
    }
    if parsed_numbers.len() != ops.len() {
        bail!(
            "got a different number of number sets ({}) and operators ({})",
            parsed_numbers.len(),
            ops.len()
        );
    }

    Ok(ops
        .into_iter()
        .zip(parsed_numbers)
        .map(|(op, numbers)| MathProblem { numbers, op })
        .map(|problem| problem.compute())
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED: &str = include_str!("../input/day06_provided.txt");

    #[test]
    fn part1_provided() {
        assert_eq!(part1(PROVIDED).unwrap(), 4277556);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 5316572080628);
    }

    #[test]
    fn part2_provided() {
        assert_eq!(part2(PROVIDED).unwrap(), 3263827);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT).unwrap(), 11299263623062);
    }
}
