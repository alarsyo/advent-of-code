use std::collections::HashMap;
use std::fmt::Write;
use std::ops::RangeInclusive;

use anyhow::{Context, Result};

const INPUT: &str = include_str!("../input/day16.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;
    writeln!(res, "part 2: {}", part2(INPUT)?)?;

    Ok(res)
}

fn part1(input: &str) -> Result<u64> {
    let (fields, _, tickets) = parse_input(input)?;

    let fields_vec = fields.values().collect::<Vec<_>>();

    Ok(tickets
        .iter()
        .map(|t| t.invalid_values(&fields_vec))
        .flatten()
        .sum())
}

fn assign_field_positions(
    fields: HashMap<&str, Field>,
    tickets: Vec<Ticket>,
) -> HashMap<usize, &str> {
    let fields_vec = fields.values().collect::<Vec<_>>();

    let tickets: Vec<Ticket> = tickets
        .into_iter()
        .filter(|t| t.invalid_values(&fields_vec).count() == 0)
        .collect();

    let num_values = tickets[0].values.len();

    // build list of all possibilities for each field
    let mut possibilities = fields
        .iter()
        .map(|(&name, field)| {
            let possibilities = (0..num_values)
                .into_iter()
                .filter(|i| tickets.iter().all(|t| t.valid_field(field, *i)))
                .collect();

            (name, possibilities)
        })
        .collect::<HashMap<&str, Vec<usize>>>();

    let mut fields_to_assign: Vec<&str> = fields.keys().copied().collect();
    let mut field_indices: HashMap<usize, &str> = HashMap::new();

    for _ in 0..fields.len() {
        // get field with only one possibility, and assign it
        let field = fields_to_assign
            .iter()
            .min_by_key(|&name| possibilities[name].len())
            .copied()
            .expect("fields_to_assign should never be empty in this loop");

        assert_eq!(possibilities[field].len(), 1);

        let possibility = possibilities[field][0];

        // remove position from other fields' possibilities
        possibilities.values_mut().for_each(|list| {
            if let Some(pos) = list.iter().position(|idx| *idx == possibility) {
                list.swap_remove(pos);
            }
        });

        field_indices.insert(possibility, field);
        fields_to_assign.swap_remove(fields_to_assign.iter().position(|f| *f == field).unwrap());
    }

    field_indices
}

fn part2(input: &str) -> Result<u64> {
    let (fields, my_ticket, tickets) = parse_input(input)?;

    let field_pos_matches = assign_field_positions(fields, tickets);

    Ok(my_ticket
        .values
        .iter()
        .enumerate()
        .filter(|(idx, _)| {
            let field = field_pos_matches[idx];
            field.starts_with("departure")
        })
        .map(|(_, val)| val)
        .product())
}

fn parse_input(input: &str) -> Result<(HashMap<&str, Field>, Ticket, Vec<Ticket>)> {
    let mut parts = input.split("\n\n");

    let fields_part = parts.next().context("no fields specification found")?;
    let my_ticket_part = parts.next().context("no personal ticket found")?;
    let tickets_part = parts.next().context("no list of tickets found")?;

    let fields = fields_part
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let name = parts.next().context("no name found")?;
            let field = parts.next().context("no ranges found")?.parse()?;

            Ok((name, field))
        })
        .collect::<Result<_>>()
        .context("couldn't parse fields")?;
    let my_ticket = my_ticket_part
        .lines()
        .skip(1)
        .map(str::parse)
        .next()
        .context("no second line for ticket")??;
    let tickets = tickets_part
        .lines()
        .skip(1)
        .map(str::parse)
        .collect::<Result<_>>()
        .context("couldn't parse tickets")?;

    Ok((fields, my_ticket, tickets))
}

#[derive(Debug)]
struct Ticket {
    values: Vec<u64>,
}

impl Ticket {
    fn invalid_values<'a>(&'a self, fields: &'a [&Field]) -> impl Iterator<Item = u64> + 'a {
        self.values
            .iter()
            .copied()
            .filter(move |val| !fields.iter().any(|f| f.contains(val)))
    }

    fn valid_field(&self, field: &Field, val_idx: usize) -> bool {
        field.contains(&self.values[val_idx])
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
        let mut ranges = s.split(" or ");

        let mut first_range = ranges.next().context("no first range found")?.split('-');
        let first_range_start = first_range.next().context("no bound for range")?.parse()?;
        let first_range_end = first_range.next().context("no bound for range")?.parse()?;

        let mut second_range = ranges.next().context("no second range found")?.split('-');
        let second_range_start = second_range.next().context("no bound for range")?.parse()?;
        let second_range_end = second_range.next().context("no bound for range")?.parse()?;

        Ok(Field {
            ranges: (
                first_range_start..=first_range_end,
                second_range_start..=second_range_end,
            ),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED1: &str = include_str!("../input/day16_provided1.txt");
    const PROVIDED2: &str = include_str!("../input/day16_provided2.txt");

    #[test]
    fn part1_provided() {
        assert_eq!(part1(PROVIDED1).unwrap(), 71);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 20013);
    }

    #[test]
    fn part2_provided() {
        let (fields, _, tickets) = parse_input(PROVIDED2).unwrap();

        let matches = assign_field_positions(fields, tickets);

        let expected = (&["row", "class", "seat"])
            .iter()
            .copied()
            .enumerate()
            .collect();

        assert_eq!(matches, expected);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT).unwrap(), 5977293343129);
    }
}
