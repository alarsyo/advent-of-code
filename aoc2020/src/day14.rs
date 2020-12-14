use std::collections::HashMap;
use std::fmt::Write;

use aoc::err;

const INPUT: &str = include_str!("../input/day14.txt");

pub fn run() -> aoc::Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;

    Ok(res)
}

fn part1(input: &str) -> aoc::Result<u64> {
    let mut program: Program = input.parse()?;

    program.run()?;

    Ok(program.memory_sum())
}

#[derive(Debug, Clone, Copy)]
enum MaskType {
    And,
    Or,
}

#[derive(Debug, Clone)]
struct BitMask {
    masks: Vec<(MaskType, u64)>,
}

impl BitMask {
    fn apply(&self, mut n: u64) -> u64 {
        for (mask_type, mask) in &self.masks {
            match mask_type {
                MaskType::And => n &= mask,
                MaskType::Or => n |= mask,
            }
        }

        n
    }
}

impl std::str::FromStr for BitMask {
    type Err = aoc::Error;

    fn from_str(s: &str) -> aoc::Result<Self> {
        let masks = s
            .chars()
            .rev()
            .enumerate()
            .filter_map(|(idx, c)| match c {
                '1' => {
                    let m = 1 << idx;
                    Some(Ok((MaskType::Or, m)))
                }
                '0' => {
                    let m = !(1 << idx);
                    Some(Ok((MaskType::And, m)))
                }
                'X' => None,
                _ => Some(Err(err!("unknown character in mask: `{}`", c))),
            })
            .collect::<aoc::Result<_>>()?;

        Ok(BitMask { masks })
    }
}

#[derive(Debug)]
enum Instruction {
    MemWrite { offset: usize, value: u64 },
    ChangeMask(BitMask),
}

impl std::str::FromStr for Instruction {
    type Err = aoc::Error;

    fn from_str(s: &str) -> aoc::Result<Self> {
        let mut words = s.split(' ');

        let first = words
            .next()
            .ok_or_else(|| err!("missing first word in instruction"))?;
        let second = words
            .next()
            .ok_or_else(|| err!("missing second word in instruction"))?;
        let third = words
            .next()
            .ok_or_else(|| err!("missing third word in instruction"))?;

        if second != "=" {
            return Err(err!("expected `=` as second word in instruction: `{}`", s));
        }

        if first == "mask" {
            Ok(Self::ChangeMask(third.parse()?))
        } else {
            let left_bracket = first
                .find('[')
                .ok_or_else(|| err!("couldn't find bracket in memory instruction"))?;
            let right_bracket = first
                .find(']')
                .ok_or_else(|| err!("couldn't find bracket in memory instruction"))?;

            let offset = first[(left_bracket + 1)..right_bracket]
                .parse()
                .map_err(|e| err!("couldn't parse memory offset: `{}`", e))?;

            let value = third
                .parse()
                .map_err(|e| err!("couldn't parse memory offset: `{}`", e))?;

            Ok(Self::MemWrite { offset, value })
        }
    }
}

struct Program {
    instructions: Vec<Instruction>,
    memory: HashMap<usize, u64>,
    current_mask: Option<BitMask>,
}

impl Program {
    fn run(&mut self) -> aoc::Result<()> {
        for inst in &self.instructions {
            match inst {
                Instruction::ChangeMask(bitmask) => self.current_mask = Some(bitmask.clone()),

                Instruction::MemWrite { offset, value } => match &self.current_mask {
                    Some(bitmask) => {
                        self.memory.insert(*offset, bitmask.apply(*value));
                    }
                    None => {
                        return Err(err!("tried to execute MemWrite but mask isn't initialized"))
                    }
                },
            }
        }

        Ok(())
    }

    fn memory_sum(&self) -> u64 {
        self.memory.iter().map(|(_, value)| value).sum()
    }
}

impl std::str::FromStr for Program {
    type Err = aoc::Error;

    fn from_str(s: &str) -> aoc::Result<Self> {
        let instructions = s
            .lines()
            .map(|line| line.parse())
            .collect::<aoc::Result<_>>()?;

        Ok(Program {
            instructions,
            memory: HashMap::new(),
            current_mask: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED: &str = include_str!("../input/day14_provided.txt");

    #[test]
    fn part1_provided() {
        assert_eq!(part1(PROVIDED).unwrap(), 165);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 4297467072083);
    }
}
