use std::fmt::Write;

use anyhow::Result;

const INPUT: &str = include_str!("../input/day15.txt");
const PART1_TURNS: usize = 2020;
const PART2_TURNS: usize = 30_000_000;

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;
    writeln!(res, "part 2: {}", part2(INPUT)?)?;

    Ok(res)
}

fn part1(input: &str) -> Result<u64> {
    let starting_numbers = input
        .trim_end()
        .split(',')
        .map(|num| num.parse().map_err(anyhow::Error::new))
        .collect::<Result<Vec<u64>>>()?;

    Ok(get_last_spoken_number(&starting_numbers, PART1_TURNS))
}

fn part2(input: &str) -> Result<u64> {
    let starting_numbers = input
        .trim_end()
        .split(',')
        .map(|num| num.parse().map_err(anyhow::Error::new))
        .collect::<Result<Vec<u64>>>()?;

    Ok(get_last_spoken_number(&starting_numbers, PART2_TURNS))
}

fn get_last_spoken_number(starting_numbers: &[u64], game_length: usize) -> u64 {
    let mut spoken_numbers = Vec::new();
    spoken_numbers.resize_with(game_length, Default::default);
    let mut next_number: u64 = 0;
    let mut turn: usize = 1;

    for num in starting_numbers {
        let last_turn = speak_number(*num, turn, &mut spoken_numbers);
        match last_turn {
            0 => next_number = 0,
            prev_turn => next_number = (turn - prev_turn) as u64,
        }

        turn += 1;
    }

    let mut last_number: u64 = 0;

    while turn <= game_length {
        // store the number we're about to speak for the solution
        last_number = next_number;

        // get the previous time this number was spoken, if any
        let last_turn = speak_number(next_number, turn, &mut spoken_numbers);
        match last_turn {
            // it's the first time it appeared, we'll say 0 next turn
            0 => next_number = 0,
            // if it was spoken before, the number we'll say next turn is the difference between the
            // two turns where it was last spoken
            prev_turn => next_number = (turn - prev_turn) as u64,
        }

        turn += 1
    }

    last_number
}

/// Inserts the turn a number was spoken in the map. Returns the previous turn the number was
/// spoken, if any
fn speak_number(n: u64, turn: usize, spoken_numbers: &mut [usize]) -> usize {
    let res = spoken_numbers[n as usize];

    spoken_numbers[n as usize] = turn;

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED: &str = include_str!("../input/day15_provided.txt");

    #[test]
    fn part1_provided() {
        let expected = [436, 1, 10, 27, 78, 438, 1836];

        for (line, expected) in PROVIDED.lines().zip(expected.iter()) {
            assert_eq!(part1(line).unwrap(), *expected);
        }
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 447);
    }

    #[test]
    #[ignore]
    fn part2_provided() {
        let expected = [175594, 2578, 3544142, 261214, 6895259, 18, 362];

        for (line, expected) in PROVIDED.lines().zip(expected.iter()) {
            assert_eq!(part2(line).unwrap(), *expected);
        }
    }

    #[test]
    #[ignore]
    fn part2_real() {
        assert_eq!(part2(INPUT).unwrap(), 11721679);
    }
}
