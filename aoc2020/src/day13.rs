use std::fmt::Write;

use aoc::err;

const INPUT: &str = include_str!("../input/day13.txt");
const PROVIDED: &str = include_str!("../input/day13_provided.txt");

pub fn run() -> aoc::Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;
    writeln!(res, "part 2: {}", part2(INPUT)?)?;

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

fn part2(input: &str) -> aoc::Result<u64> {
    let mut lines = input.lines();

    // we don't need the first line anymore, skip it
    lines.next().ok_or_else(|| err!("input was empty"))?;

    let bus_ids: Vec<(u64, u64)> = lines
        .next()
        .ok_or_else(|| err!("no second line"))?
        .split(',')
        .enumerate()
        .filter_map(|(idx, num)| {
            if num == "x" {
                None
            } else {
                Some((idx as u64, num.parse::<u64>().map_err(|e| err!("{}", e))))
            }
        })
        .map(|(idx, res)| match res {
            Ok(num) => Ok((idx, num)),
            Err(e) => Err(e),
        })
        .collect::<aoc::Result<_>>()?;

    // previous constraints is empty for now
    let mut current_solution = 0;
    let mut step = 1;

    for constraint in bus_ids {
        while !satisfies_constraint(current_solution, constraint) {
            current_solution += step;
        }

        let (_, divisor) = constraint;

        step *= divisor;
    }

    Ok(current_solution)
}

fn satisfies_constraint(solution: u64, (remainder, divisor): (u64, u64)) -> bool {
    ((solution + remainder) % divisor) == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_provided() {
        assert_eq!(part1(PROVIDED).unwrap(), 295);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 3269);
    }

    #[test]
    fn part2_provided() {
        assert_eq!(part2(PROVIDED).unwrap(), 1068781);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT).unwrap(), 672754131923874);
    }
}
