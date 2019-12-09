use std::fmt::Write;

use aoc::err;
use aoc::Result;

use crate::intcode::{parse_memory, Intcode};

const INPUT: &str = include_str!("../input/day09.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    let memory = parse_memory(INPUT)?;

    writeln!(res, "part 1: {}", part1(memory.clone())?)?;
    writeln!(res, "part 2: {}", part2(memory)?)?;

    Ok(res)
}

fn part1(memory: Vec<i64>) -> Result<i64> {
    let mut intcode = Intcode::with_memory(memory);

    intcode.add_input(1);
    intcode.run()?;
    intcode
        .get_last_output()
        .ok_or_else(|| err!("intcode output was empty!"))
}

fn part2(memory: Vec<i64>) -> Result<i64> {
    let mut intcode = Intcode::with_memory(memory);

    intcode.add_input(2);
    intcode.run()?;
    intcode
        .get_last_output()
        .ok_or_else(|| err!("intcode output was empty!"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED1: &str = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
    const PROVIDED2: &str = "1102,34915192,34915192,7,4,7,99,0";
    const PROVIDED3: &str = "104,1125899906842624,99";

    #[test]
    fn part1_provided() {
        let memory = parse_memory(PROVIDED1).unwrap();
        let mut intcode = Intcode::with_memory(memory.clone());
        intcode.run().unwrap();
        assert_eq!(intcode.output, memory, "should output a copy of itself");

        let mut intcode = Intcode::new(PROVIDED2).unwrap();
        intcode.run().unwrap();
        assert!(
            intcode.output[0] > 999_999_999_999_999,
            "should be 16 digit number"
        );

        let mut intcode = Intcode::new(PROVIDED3).unwrap();
        intcode.run().unwrap();
        assert!(intcode.output[0] == 1_125_899_906_842_624);
    }

    #[test]
    fn part1_real() {
        let memory = parse_memory(INPUT).unwrap();
        assert_eq!(part1(memory).unwrap(), 3_533_056_970);
    }

    #[test]
    fn part2_real() {
        let memory = parse_memory(INPUT).unwrap();
        assert_eq!(part2(memory).unwrap(), 72_852);
    }
}
