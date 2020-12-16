use std::fmt::Write;
use std::ops::RangeInclusive;

use anyhow::{Context, Result};

const INPUT: &str = include_str!("../input/day16.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;

    Ok(res)
}

fn part1(input: &str) -> Result<u64> {
    let (fields, _, tickets) = parse_input(input)?;

    Ok(tickets
        .iter()
        .map(|t| t.invalid_values(&fields))
        .flatten()
        .sum())
}

fn parse_input(input: &str) -> Result<(Vec<Field>, Ticket, Vec<Ticket>)> {
    let mut parts = input.split("\n\n");

    let fields_part = parts.next().context("no fields specification found")?;
    let my_ticket_part = parts.next().context("no personal ticket found")?;
    let tickets_part = parts.next().context("no list of tickets found")?;

    let fields = fields_part
        .lines()
        .map(|line| line.parse())
        .collect::<Result<_>>()
        .context("couldn't parse fields")?;
    let my_ticket = my_ticket_part
        .lines()
        .skip(1)
        .map(|line| line.parse())
        .next()
        .context("no second line for ticket")??;
    let tickets = tickets_part
        .lines()
        .skip(1)
        .map(|line| line.parse())
        .collect::<Result<_>>()
        .context("couldn't parse tickets")?;

    Ok((fields, my_ticket, tickets))
}

#[derive(Debug)]
struct Ticket {
    values: Vec<u64>,
}

impl Ticket {
    fn invalid_values(&self, fields: &[Field]) -> Vec<u64> {
        self.values
            .iter()
            .filter(|val| !fields.iter().any(|f| f.contains(*val)))
            .copied()
            .collect()
    }
}

impl std::str::FromStr for Ticket {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let values = s
            .split(',')
            .map(|num| {
                num.parse()
                    .with_context(|| format!("couldn't parse `{}`", num))
            })
            .collect::<Result<_>>()?;

        Ok(Ticket { values })
    }
}

#[derive(Debug)]
struct Field {
    name: String,
    ranges: (RangeInclusive<u64>, RangeInclusive<u64>),
}

impl Field {
    fn contains(&self, val: &u64) -> bool {
        self.ranges.0.contains(val) || self.ranges.1.contains(val)
    }
}

impl std::str::FromStr for Field {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.split(": ");

        let name = parts.next().context("no name found")?.to_string();

        let mut ranges = parts.next().context("no ranges found")?.split(" or ");

        let mut range1 = ranges.next().context("no first range found")?.split('-');
        let range1_start = range1.next().context("no bound for range")?.parse()?;
        let range1_end = range1.next().context("no bound for range")?.parse()?;

        let mut range2 = ranges.next().context("no second range found")?.split('-');
        let range2_start = range2.next().context("no bound for range")?.parse()?;
        let range2_end = range2.next().context("no bound for range")?.parse()?;

        Ok(Field {
            name,
            ranges: (range1_start..=range1_end, range2_start..=range2_end),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED: &str = include_str!("../input/day16_provided.txt");

    #[test]
    fn part1_provided() {
        assert_eq!(part1(PROVIDED).unwrap(), 71);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 20013);
    }
}
