use std::collections::VecDeque;
use std::fmt::Write;

use aoc::err;
use aoc::Result;

use crate::intcode::{parse_memory, Intcode};

const INPUT: &str = include_str!("../input/day07.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;
    writeln!(res, "part 2: {}", part2(INPUT)?)?;

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

fn part2(input: &str) -> Result<i64> {
    let memory = parse_memory(input)?;

    let combinations = permutations(&[5, 6, 7, 8, 9]);

    let mut res = 0;
    for combination in combinations {
        let intcodes = &mut [
            Intcode::with_memory(memory.clone()),
            Intcode::with_memory(memory.clone()),
            Intcode::with_memory(memory.clone()),
            Intcode::with_memory(memory.clone()),
            Intcode::with_memory(memory.clone()),
        ];

        for (phase, intcode) in combination.iter().zip(intcodes.iter_mut()) {
            intcode.add_input(*phase);
        }
        intcodes[0].add_input(0);

        let mut signal = None;
        let mut num_halted = 0;
        loop {
            for i in 0..(intcodes.len() - 1) {
                let (first, second) = intcodes.split_at_mut(i + 1);
                let intcode = &mut first[i];
                let next = &mut second[0];

                let halted = intcode.run_and_wait()?;
                if halted {
                    num_halted += 1;
                }

                for out in intcode.output.iter() {
                    next.add_input(*out);
                }
                intcode.output.clear();
            }

            let last_index = intcodes.len() - 1;
            let (first, second) = intcodes.split_at_mut(last_index);
            let first = &mut first[0];
            let last = &mut second[0];
            let halted = last.run_and_wait()?;

            if halted {
                let out = last
                    .output
                    .last()
                    .copied()
                    .ok_or_else(|| err!("last amplifier halted without output"))?;
                signal = Some(out);
            } else {
                for out in last.output.iter() {
                    first.add_input(*out);
                }
                last.output.clear();
            }

            if let Some(signal) = signal {
                res = std::cmp::max(res, signal);
                break;
            }

            if num_halted >= 4 {
                return Err(err!("all non final amplifiers halted, feedback loop stuck"));
            }
        }
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

    const PROVIDED4: &str =
        "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
    const PROVIDED5: &str = "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10";

    #[test]
    fn part2_provided() {
        assert_eq!(part2(PROVIDED4).unwrap(), 139629729);
        assert_eq!(part2(PROVIDED5).unwrap(), 18216);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT).unwrap(), 4215746);
    }
}
