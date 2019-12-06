use std::fmt::Write;

use aoc::err;
use aoc::Result;

const INPUT: &str = include_str!("../input/day02.txt");
const PART2_EXPECTED: usize = 19_690_720;

fn parse_intcode(input: &str) -> Result<Vec<usize>> {
    input
        .trim_end()
        .split(',')
        .map(|x| x.parse().map_err(|e| err!("couldn't parse int: {}", e)))
        .collect()
}

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    let intcode = parse_intcode(INPUT)?;

    writeln!(res, "part 1: {}", part1(&mut intcode.clone())?)?;
    writeln!(res, "part 2: {}", part2(&intcode, PART2_EXPECTED)?)?;

    Ok(res)
}

fn eval(intcode: &mut [usize]) -> Result<()> {
    let mut pc = 0;

    while intcode[pc] != 99 {
        let op1 = intcode[pc + 1];
        let op2 = intcode[pc + 2];
        let res = intcode[pc + 3];

        match intcode[pc] {
            1 => intcode[res] = intcode[op1] + intcode[op2],
            2 => intcode[res] = intcode[op1] * intcode[op2],
            _ => return Err(err!("unknown opcode: {}", intcode[pc])),
        };

        pc += 4;
    }

    Ok(())
}

fn part1(input: &mut Vec<usize>) -> Result<usize> {
    input[1] = 12;
    input[2] = 2;

    eval(input)?;

    Ok(input[0])
}

fn part2(input: &[usize], res: usize) -> Result<usize> {
    for (noun, verb) in (0..=99).flat_map(|noun| (0..=99).map(move |verb| (noun, verb))) {
        let mut test_input = input.to_vec();
        test_input[1] = noun;
        test_input[2] = verb;

        eval(&mut test_input)?;

        if test_input[0] == res {
            return Ok(noun * 100 + verb);
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
        let mut intcode = parse_intcode("1,9,10,3,2,3,11,0,99,30,40,50").unwrap();
        eval(&mut intcode).unwrap();
        assert_eq!(intcode, &[3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);

        let mut intcode = parse_intcode("1,0,0,0,99").unwrap();
        eval(&mut intcode).unwrap();
        assert_eq!(intcode, &[2, 0, 0, 0, 99]);

        let mut intcode = parse_intcode("2,3,0,3,99").unwrap();
        eval(&mut intcode).unwrap();
        assert_eq!(intcode, &[2, 3, 0, 6, 99]);

        let mut intcode = parse_intcode("2,4,4,5,99,0").unwrap();
        eval(&mut intcode).unwrap();
        assert_eq!(intcode, &[2, 4, 4, 5, 99, 9801]);

        let mut intcode = parse_intcode("1,1,1,4,99,5,6,0,99").unwrap();
        eval(&mut intcode).unwrap();
        assert_eq!(intcode, &[30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn part1_real() {
        let mut intcode = parse_intcode(INPUT).unwrap();
        assert_eq!(part1(&mut intcode).unwrap(), 6568671);
    }

    #[test]
    fn part2_real() {
        let intcode = parse_intcode(INPUT).unwrap();
        assert_eq!(part2(&intcode, PART2_EXPECTED).unwrap(), 3951);
    }
}
