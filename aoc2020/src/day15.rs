use std::collections::HashMap;
use std::fmt::Write;

use anyhow::Result;

const INPUT: &str = include_str!("../input/day15.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;

    Ok(res)
}

fn part1(input: &str) -> Result<u64> {
    let starting_numbers = input.trim_end()
        .split(',')
        .map(|num| num.parse().map_err(anyhow::Error::new))
        .collect::<Result<Vec<u64>>>()?;

    let mut spoken_numbers = HashMap::new();
    let mut next_number: u64 = 0;
    let mut turn: usize = 0;

    for num in starting_numbers {
        let last_turn = speak_number(num, turn, &mut spoken_numbers);
        match last_turn {
            Some(prev_turn) => next_number = (turn - prev_turn) as u64,
            None => next_number = 0,
        }

        turn += 1;
    }

    let mut last_number: u64 = 0;

    while turn < 2020 {
        // store the number we're about to speak for the solution
        last_number = next_number;

        // get the previous time this number was spoken, if any
        let last_turn = speak_number(next_number, turn, &mut spoken_numbers);
        match last_turn {
            // if it was spoken before, the number we'll say next turn is the difference between the
            // two turns where it was last spoken
            Some(prev_turn) => next_number = (turn - prev_turn) as u64,
            // otherwise we'll say 0 next turn
            None => next_number = 0,
        }

        turn += 1
    }

    Ok(last_number)
}

/// Inserts the turn a number was spoken in the map. Returns the previous turn the number was
/// spoken, if any
fn speak_number(n: u64, turn: usize, spoken_numbers: &mut HashMap<u64, usize>) -> Option<usize> {
    let res = spoken_numbers.get(&n).copied();

    spoken_numbers.insert(n, turn);

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED: &str = include_str!("../input/day15_provided.txt");

    #[test]
    fn part1_provided() {
        assert_eq!(part1(PROVIDED).unwrap(), 436);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 447);
    }
}
