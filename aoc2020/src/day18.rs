use std::fmt::Write;

use anyhow::{anyhow, Result};

use nom::{
    branch::alt,
    bytes::complete::take_while1,
    character::complete::{char, one_of},
    combinator::{eof, map_res},
    multi::fold_many0,
    sequence::{delimited, pair, terminated},
    IResult, Parser,
};

const INPUT: &str = include_str!("../input/day18.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;

    Ok(res)
}

fn part1(input: &str) -> Result<u64> {
    input
        .lines()
        .map(|line| line.parse::<Expr>().map(|e| e.eval()))
        .sum()
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Addition,
    Multiplication,
}

#[derive(Debug, Clone)]
enum Expr {
    Num(u64),
    Op(Box<Expr>, Operator, Box<Expr>),
}

impl Expr {
    fn eval(&self) -> u64 {
        match self {
            Self::Num(val) => *val,
            Self::Op(lhs, op, rhs) => match op {
                Operator::Addition => lhs.eval() + rhs.eval(),
                Operator::Multiplication => lhs.eval() * rhs.eval(),
            },
        }
    }
}

fn operator(input: &str) -> IResult<&str, Operator> {
    one_of("+*")
        .map(|op| match op {
            '+' => Operator::Addition,
            '*' => Operator::Multiplication,
            _ => unreachable!(),
        })
        .parse(input)
}

fn operator_expr(input: &str) -> IResult<&str, Expr> {
    let (i, first_term) = term(input)?;

    fold_many0(
        pair(delimited(char(' '), operator, char(' ')), term),
        first_term,
        |acc, (op, val)| Expr::Op(Box::new(acc), op, Box::new(val)),
    )(i)
}

fn num(input: &str) -> IResult<&str, Expr> {
    map_res(take_while1(|c: char| c.is_digit(10)), |res: &str| {
        res.parse().map(Expr::Num)
    })(input)
}

fn term(input: &str) -> IResult<&str, Expr> {
    alt((delimited(char('('), expr, char(')')), num))(input)
}

fn expr(input: &str) -> IResult<&str, Expr> {
    alt((operator_expr, term))(input)
}

impl std::str::FromStr for Expr {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (_, res) = terminated(expr, eof)(s).map_err(|_| anyhow!("couldn't parse expr"))?;

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_provided() {
        let tests = &[
            ("1 + 2 * 3 + 4 * 5 + 6", 71),
            ("1 + (2 * 3) + (4 * (5 + 6))", 51),
            ("2 * 3 + (4 * 5)", 26),
            ("5 + (8 * 3 + 9 + 3 * 4 * 3)", 437),
            ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 12240),
            ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 13632),
        ];

        for (expr, expected) in tests {
            let expr: Expr = expr.parse().unwrap();
            assert_eq!(expr.eval(), *expected);
        }
    }
}
