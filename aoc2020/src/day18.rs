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
    writeln!(res, "part 2: {}", part2(INPUT)?)?;

    Ok(res)
}

fn part1(input: &str) -> Result<u64> {
    input
        .lines()
        .map(|line| {
            terminated(expr, eof)(line)
                .map_err(|_| anyhow!("couldn't parse expr"))
                .map(|(_, e)| e.eval())
        })
        .sum()
}

fn part2(input: &str) -> Result<u64> {
    input
        .lines()
        .map(|line| {
            terminated(plus_priority, eof)(line)
                .map_err(|_| anyhow!("couldn't parse expr"))
                .map(|(_, e)| e.eval())
        })
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
        move || first_term.clone(),
        |acc, (op, val)| Expr::Op(Box::new(acc), op, Box::new(val)),
    )(i)
}

fn plus(input: &str) -> IResult<&str, Expr> {
    let (i, first_term) = term_plus_priority(input)?;

    fold_many0(
        pair(
            delimited(char(' '), char('+'), char(' ')),
            term_plus_priority,
        ),
        move || first_term.clone(),
        |acc, (_, val)| Expr::Op(Box::new(acc), Operator::Addition, Box::new(val)),
    )(i)
}

fn mul(input: &str) -> IResult<&str, Expr> {
    let (i, first_factor) = plus(input)?;

    fold_many0(
        pair(delimited(char(' '), char('*'), char(' ')), plus),
        move || first_factor.clone(),
        |acc, (_, val)| Expr::Op(Box::new(acc), Operator::Multiplication, Box::new(val)),
    )(i)
}

fn num(input: &str) -> IResult<&str, Expr> {
    map_res(take_while1(|c: char| c.is_digit(10)), |res: &str| {
        res.parse().map(Expr::Num)
    })(input)
}

fn paren(input: &str) -> IResult<&str, Expr> {
    delimited(char('('), expr, char(')'))(input)
}

fn paren_plus_priority(input: &str) -> IResult<&str, Expr> {
    delimited(char('('), plus_priority, char(')'))(input)
}

fn term(input: &str) -> IResult<&str, Expr> {
    alt((paren, num))(input)
}

fn term_plus_priority(input: &str) -> IResult<&str, Expr> {
    alt((paren_plus_priority, num))(input)
}

fn expr(input: &str) -> IResult<&str, Expr> {
    alt((operator_expr, term))(input)
}

fn plus_priority(input: &str) -> IResult<&str, Expr> {
    alt((mul, term_plus_priority))(input)
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

        for (exp, expected) in tests {
            let (_, exp) = terminated(expr, eof)(exp).unwrap();
            assert_eq!(exp.eval(), *expected);
        }
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 800602729153);
    }

    #[test]
    fn part2_provided() {
        let tests = &[
            ("1 + 2 * 3 + 4 * 5 + 6", 231),
            ("1 + (2 * 3) + (4 * (5 + 6))", 51),
            ("2 * 3 + (4 * 5)", 46),
            ("5 + (8 * 3 + 9 + 3 * 4 * 3)", 1445),
            ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 669060),
            ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 23340),
        ];

        for (exp, expected) in tests {
            let (_, exp) = terminated(plus_priority, eof)(exp).unwrap();
            assert_eq!(exp.eval(), *expected);
        }
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT).unwrap(), 92173009047076);
    }
}
