use std::fmt::Write;

use anyhow::Result;

const INPUT: &str = include_str!("../input/day00.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;

    Ok(res)
}

fn part1(input: &str) -> Result<&str> {
    Ok(input)
}
