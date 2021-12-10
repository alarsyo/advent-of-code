use std::fmt::Write;

use anyhow::{bail, Result};

const INPUT: &str = include_str!("../input/day10.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;

    Ok(res)
}

fn part1(input: &str) -> Result<u64> {
    let lines = input
        .lines()
        .map(str::parse::<Line>)
        .collect::<Result<Vec<_>>>()?;

    Ok(lines.iter().map(Line::corrupt_score).sum())
}

enum SymbolState {
    Open(Symbol),
    Close(Symbol),
}

impl TryFrom<char> for SymbolState {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self> {
        match value {
            '(' | '[' | '{' | '<' => Ok(SymbolState::Open(value.try_into().unwrap())),
            ')' | ']' | '}' | '>' => Ok(SymbolState::Close(value.try_into().unwrap())),
            _ => bail!("invalid char for symbol"),
        }
    }
}

#[derive(PartialEq, Eq)]
enum Symbol {
    Parenthesis,  // ()
    Bracket,      // []
    Brace,        // {}
    AngleBracket, // <>
}

impl Symbol {
    fn score(&self) -> u64 {
        match self {
            Symbol::Parenthesis => 3,
            Symbol::Bracket => 57,
            Symbol::Brace => 1197,
            Symbol::AngleBracket => 25137,
        }
    }
}

impl TryFrom<char> for Symbol {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self> {
        match value {
            '(' | ')' => Ok(Symbol::Parenthesis),
            '[' | ']' => Ok(Symbol::Bracket),
            '{' | '}' => Ok(Symbol::Brace),
            '<' | '>' => Ok(Symbol::AngleBracket),
            _ => bail!("invalid char for symbol"),
        }
    }
}

struct Line {
    symbols: Vec<SymbolState>,
}

impl Line {
    fn corrupt_score(&self) -> u64 {
        let mut stack = Vec::new();

        for state in &self.symbols {
            match state {
                SymbolState::Open(symbol) => stack.push(symbol),
                SymbolState::Close(symbol) => match stack.pop() {
                    Some(other_symbol) if symbol == other_symbol => continue,
                    _ => return symbol.score(),
                },
            }
        }

        0
    }
}

impl std::str::FromStr for Line {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Line {
            symbols: s
                .trim()
                .chars()
                .map(TryFrom::try_from)
                .collect::<Result<Vec<_>>>()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED: &str = include_str!("../input/day10_provided.txt");

    #[test]
    fn part1_provided() {
        assert_eq!(part1(PROVIDED).unwrap(), 26397);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 394647);
    }
}
