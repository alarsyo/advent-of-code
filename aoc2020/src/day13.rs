use std::fmt::Write;

use aoc::err;

const INPUT: &str = include_str!("../input/day13.txt");

pub fn run() -> aoc::Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;

    Ok(res)
}

fn part1(input: &str) -> aoc::Result<u64> {
    let mut lines = input.lines();

    let earliest_timestamp = lines
        .next()
        .ok_or_else(|| err!("input was empty"))?
        .parse::<u64>()
        .map_err(|e| err!("couldn't parse first line: {}", e))?;

    let bus_ids = lines
        .next()
        .ok_or_else(|| err!("no second line"))?
        .split(',')
        .filter_map(|num| {
            if num == "x" {
                None
            } else {
                Some(num.parse::<u64>().map_err(|e| err!("{}", e)))
            }
        })
        .collect::<aoc::Result<Vec<_>>>()?;

    let (bus_id, earliest_departure) = bus_ids
        .iter()
        .map(|id| {
            let next_departure = ((earliest_timestamp / id) * id) + id;
            (id, next_departure)
        })
        .min_by_key(|(_, next_departure)| *next_departure)
        .unwrap();

    Ok(bus_id * (earliest_departure - earliest_timestamp))
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED: &str = include_str!("../input/day13_provided.txt");

    #[test]
    fn part1_provided() {
        assert_eq!(part1(PROVIDED).unwrap(), 295);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 3269);
    }
}
