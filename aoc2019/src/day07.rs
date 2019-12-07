use std::collections::VecDeque;
use std::fmt::Write;

use aoc::err;
use aoc::Result;

use crate::intcode::{parse_memory, Intcode};

const INPUT: &str = include_str!("../input/day07.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;

    Ok(res)
}

fn permutations_rec(used: &mut Vec<i64>, available: &mut VecDeque<i64>, res: &mut Vec<Vec<i64>>) {
    if available.is_empty() {
        res.push(used.clone());
    } else {
        for _ in 0..available.len() {
            used.push(available.pop_front().unwrap());
            permutations_rec(used, available, res);
            available.push_back(used.pop().unwrap());
        }
    }
}

fn permutations(array: &[i64]) -> Vec<Vec<i64>> {
    let mut res = Vec::new();

    permutations_rec(
        &mut Vec::new(),
        &mut VecDeque::from(array.to_vec()),
        &mut res,
    );

    res
}

fn part1(input: &str) -> Result<i64> {
    let memory = parse_memory(input)?;

    let combinations = permutations(&[0, 1, 2, 3, 4]);

    let mut res = 0;
    for combination in combinations {
        let mut output = 0;
        for phase in combination.iter() {
            let mut intcode = Intcode::with_memory(memory.clone());

            intcode.add_input(*phase);
            intcode.add_input(output);

            intcode.run()?;

            output = intcode
                .get_day05_output()
                .ok_or_else(|| err!("no output at end of pipeline!"))?;
        }

        res = std::cmp::max(res, output);
    }

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED1: &str = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
    const PROVIDED2: &str =
        "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
    const PROVIDED3: &str = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";

    #[test]
    fn part1_provided() {
        assert_eq!(part1(PROVIDED1).unwrap(), 43210);
        assert_eq!(part1(PROVIDED2).unwrap(), 54321);
        assert_eq!(part1(PROVIDED3).unwrap(), 65210);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 844468);
    }
}
