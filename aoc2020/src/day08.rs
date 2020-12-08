use std::collections::HashSet;
use std::fmt::Write;

use aoc::err;

const INPUT: &str = include_str!("../input/day08.txt");

pub fn run() -> aoc::Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;

    Ok(res)
}

fn part1(input: &str) -> aoc::Result<i64> {
    let instructions = input
        .lines()
        .map(|line| line.parse())
        .collect::<aoc::Result<Vec<Instruction>>>()?;

    let mut interpreter = Interpreter::new(instructions);

    Ok(match interpreter.run() {
        ExitStatus::InfiniteLoop(value) => value,
        ExitStatus::End(_) => return Err(err!("interpreter doesn't have an infinite loop")),
    })
}

struct Interpreter {
    idx: usize,
    accumulator: i64,
    memory: Vec<Instruction>,
}

enum ExitStatus {
    InfiniteLoop(i64),
    End(i64),
}

impl Interpreter {
    fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            idx: 0,
            accumulator: 0,
            memory: instructions,
        }
    }

    fn step(&mut self) {
        match self.memory[self.idx] {
            Instruction::Acc(arg) => {
                self.accumulator += arg;
                self.idx += 1;
            }
            Instruction::Jmp(offset) => self.idx = self.idx.wrapping_add(offset as usize),
            Instruction::Nop => self.idx += 1,
        }
    }

    fn run(&mut self) -> ExitStatus {
        let mut set = HashSet::new();

        while self.idx < self.memory.len() {
            if !set.insert(self.idx) {
                return ExitStatus::InfiniteLoop(self.accumulator);
            }

            self.step();
        }

        return ExitStatus::End(self.accumulator);
    }
}

enum Instruction {
    Acc(i64),
    Jmp(i64),
    Nop,
}

impl std::str::FromStr for Instruction {
    type Err = aoc::Error;

    fn from_str(s: &str) -> aoc::Result<Self> {
        let space = s.find(' ').ok_or_else(|| err!("couldn't split on space"))?;

        let inst = &s[..space];
        let arg = s[(space + 1)..]
            .parse()
            .map_err(|e| err!("couldn't parse argument for instruction: {}", e))?;

        Ok(match inst {
            "acc" => Self::Acc(arg),
            "jmp" => Self::Jmp(arg),
            "nop" => Self::Nop,
            _ => return Err(err!("unrecognized instruction `{}`", inst)),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED: &str = include_str!("../input/day08_provided.txt");

    #[test]
    fn part1_provided() {
        assert_eq!(part1(PROVIDED).unwrap(), 5);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 1675);
    }
}
