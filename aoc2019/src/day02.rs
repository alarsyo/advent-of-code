use std::fmt::Write;

use aoc::err;
use aoc::Result;

use crate::intcode::{parse_memory, Intcode};

const INPUT: &str = include_str!("../input/day02.txt");
const PART2_EXPECTED: i64 = 19_690_720;

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    let memory = parse_memory(INPUT)?;

    writeln!(res, "part 1: {}", part1(memory.clone())?)?;
    writeln!(res, "part 2: {}", part2(&memory, PART2_EXPECTED)?)?;

    Ok(res)
}

fn part1(mut input: Vec<i64>) -> Result<i64> {
    input[1] = 12;
    input[2] = 2;

    let mut intcode = Intcode::with_memory(input);
    intcode.run()?;

    intcode
        .get_day02_output()
        .ok_or_else(|| err!("intcode memory was empty!"))
}

fn part2(input: &[i64], res: i64) -> Result<i64> {
    for (noun, verb) in (0..=99).flat_map(|noun| (0..=99).map(move |verb| (noun, verb))) {
        let mut test_input = input.to_vec();
        test_input[1] = noun;
        test_input[2] = verb;

        let mut intcode = Intcode::with_memory(test_input);
        intcode.run()?;

        match intcode.get_day02_output() {
            Some(val) => {
                if val == res {
                    return Ok(noun * 100 + verb);
                }
            }
            None => return Err(err!("intcode memory was empty!")),
        }
    }

    Err(err!(
        "couldn't find noun/verb combination that produces {}",
        res
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_provided() {
        let mut intcode = Intcode::new("1,9,10,3,2,3,11,0,99,30,40,50").unwrap();
        intcode.run().unwrap();
        assert_eq!(
            &intcode.memory,
            &[3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );

        let mut intcode = Intcode::new("1,0,0,0,99").unwrap();
        intcode.run().unwrap();
        assert_eq!(&intcode.memory, &[2, 0, 0, 0, 99]);

        let mut intcode = Intcode::new("2,3,0,3,99").unwrap();
        intcode.run().unwrap();
        assert_eq!(&intcode.memory, &[2, 3, 0, 6, 99]);

        let mut intcode = Intcode::new("2,4,4,5,99,0").unwrap();
        intcode.run().unwrap();
        assert_eq!(&intcode.memory, &[2, 4, 4, 5, 99, 9801]);

        let mut intcode = Intcode::new("1,1,1,4,99,5,6,0,99").unwrap();
        intcode.run().unwrap();
        assert_eq!(&intcode.memory, &[30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn part1_real() {
        let memory = parse_memory(INPUT).unwrap();
        assert_eq!(part1(memory).unwrap(), 6568671);
    }

    #[test]
    fn part2_real() {
        let memory = parse_memory(INPUT).unwrap();
        assert_eq!(part2(&memory, PART2_EXPECTED).unwrap(), 3951);
    }
}
