use std::collections::HashMap;
use std::fmt::Write;

use anyhow::{anyhow, bail, Context, Result};

const INPUT: &str = include_str!("../input/day14.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;
    writeln!(res, "part 2: {}", part2(INPUT)?)?;

    Ok(res)
}

fn part1(input: &str) -> Result<u64> {
    let mut program: Program = input.parse()?;

    program.run_part1()?;

    Ok(program.memory_sum())
}

fn part2(input: &str) -> Result<u64> {
    let mut program: Program = input.parse()?;

    program.run_part2()?;

    Ok(program.memory_sum())
}

/// Represents the kind of mask we want to apply at a specific offset
#[derive(Debug, Clone, Copy)]
enum Mask {
    Floating,
    One,
    Zero,
}

/// An iterator over all possible values produced by applying `masks` on a single value
#[derive(Debug)]
struct FloatingIterator<'a> {
    masks: &'a [Mask],
    current_value: usize,
    done: bool,

    stack: Vec<(usize, FloatingState)>,
}

impl<'a> FloatingIterator<'a> {
    fn new(masks: &'a [Mask], mut n: usize) -> Self {
        // apply non-floating masks here, we don't want to process them in every iteration
        for (offset, mask) in masks.iter().enumerate() {
            if let Mask::One = mask {
                n |= 1 << offset;
            }
        }

        Self {
            masks,
            current_value: n,
            done: false,
            stack: Vec::new(),
        }
    }

    /// Returns the offset of the next Floating Mask it finds after offset, if any
    fn find_next_floating(masks: &[Mask], offset: usize) -> Option<usize> {
        masks
            .iter()
            .enumerate()
            .skip(offset)
            .find(|(_, m)| matches!(m, Mask::Floating))
            .map(|(idx, _)| idx)
    }
}

#[derive(Debug)]
enum FloatingState {
    Unapplied,
    AppliedZero,
    AppliedZeroAndOne,
}

impl<'a> Iterator for FloatingIterator<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        if self.stack.is_empty() {
            // initialize stack with first floating iterator
            let next = FloatingIterator::find_next_floating(self.masks, 0);

            match next {
                Some(offset) => self.stack.push((offset, FloatingState::Unapplied)),
                None => {
                    // there are no floating bits in this mask, we can return the value directly
                    self.done = true;
                    return Some(self.current_value);
                }
            }
        }

        loop {
            let (offset, state) = self.stack.last_mut().unwrap();
            match state {
                FloatingState::Unapplied => {
                    // apply the Zero mask
                    self.current_value &= !(1 << *offset);
                    *state = FloatingState::AppliedZero;
                }
                FloatingState::AppliedZero => {
                    // apply the One mask
                    self.current_value |= 1 << *offset;
                    *state = FloatingState::AppliedZeroAndOne;
                }
                FloatingState::AppliedZeroAndOne => {
                    // we've applied all possibilities for this mask, we can unwind our stack
                    self.stack.pop();

                    if self.stack.is_empty() {
                        // we've computed all possibilities, we can just stop now
                        self.done = true;
                        return None;
                    }

                    continue;
                }
            }

            // we've applied our current mask transform, now we "recur" and find the next one
            match FloatingIterator::find_next_floating(self.masks, *offset + 1) {
                Some(offset) => self.stack.push((offset, FloatingState::Unapplied)),
                None => {
                    // we were the last Floating mask, we can return the produced value
                    return Some(self.current_value);
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
struct BitMask {
    masks: Vec<Mask>,
}

impl BitMask {
    /// Function used for part 1: 'X' bits just don't do anything
    ///
    /// This discards Floating masks, returning a single modified value
    fn apply_no_floating(&self, mut n: u64) -> u64 {
        for (offset, mask) in self.masks.iter().enumerate() {
            match mask {
                Mask::Floating => {}
                Mask::One => n |= 1 << offset,
                Mask::Zero => n &= !(1 << offset),
            }
        }

        n
    }

    /// Returns an iterator over all possible values when applying the BitMask
    ///
    /// This takes into account Floating masks, which produce multiple possibilities, hence the need
    /// for an iterator
    fn apply(&self, n: usize) -> impl Iterator<Item = usize> + '_ {
        FloatingIterator::new(&self.masks, n)
    }
}

impl std::str::FromStr for BitMask {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let masks = s
            .chars()
            .rev()
            .map(|c| {
                // idx will never be higher than 36 so this is fine
                match c {
                    '1' => Ok(Mask::One),
                    '0' => Ok(Mask::Zero),
                    'X' => Ok(Mask::Floating),
                    _ => Err(anyhow!("unknown character in mask: `{}`", c)),
                }
            })
            .collect::<Result<_>>()?;

        Ok(BitMask { masks })
    }
}

#[derive(Debug)]
enum Instruction {
    MemWrite { offset: usize, value: u64 },
    ChangeMask(BitMask),
}

impl std::str::FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut words = s.split(' ');

        let first = words.next().context("missing first word in instruction")?;
        let second = words.next().context("missing second word in instruction")?;
        let third = words.next().context("missing third word in instruction")?;

        if second != "=" {
            bail!("expected `=` as second word in instruction: `{}`", s);
        }

        if first == "mask" {
            Ok(Self::ChangeMask(third.parse()?))
        } else {
            let left_bracket = first
                .find('[')
                .context("couldn't find bracket in memory instruction")?;
            let right_bracket = first
                .find(']')
                .context("couldn't find bracket in memory instruction")?;

            let offset = first[(left_bracket + 1)..right_bracket]
                .parse()
                .context("couldn't parse memory offset")?;

            let value = third.parse().context("couldn't parse memory offset")?;

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
    fn run_part1(&mut self) -> Result<()> {
        for inst in &self.instructions {
            match inst {
                Instruction::ChangeMask(bitmask) => self.current_mask = Some(bitmask.clone()),

                Instruction::MemWrite { offset, value } => match &self.current_mask {
                    Some(bitmask) => {
                        self.memory
                            .insert(*offset, bitmask.apply_no_floating(*value));
                    }
                    None => {
                        bail!("tried to execute MemWrite but mask isn't initialized")
                    }
                },
            }
        }

        Ok(())
    }

    fn run_part2(&mut self) -> Result<()> {
        for inst in &self.instructions {
            match inst {
                Instruction::ChangeMask(bitmask) => self.current_mask = Some(bitmask.clone()),

                Instruction::MemWrite { offset, value } => match &self.current_mask {
                    Some(bitmask) => {
                        for offset in bitmask.apply(*offset) {
                            self.memory.insert(offset, *value);
                        }
                    }
                    None => {
                        bail!("tried to execute MemWrite but mask isn't initialized")
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
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let instructions = s.lines().map(|line| line.parse()).collect::<Result<_>>()?;

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

    const PROVIDED1: &str = include_str!("../input/day14_provided1.txt");
    const PROVIDED2: &str = include_str!("../input/day14_provided2.txt");

    #[test]
    fn part1_provided() {
        assert_eq!(part1(PROVIDED1).unwrap(), 165);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 4297467072083);
    }

    #[test]
    fn part2_provided() {
        assert_eq!(part2(PROVIDED2).unwrap(), 208);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT).unwrap(), 5030603328768);
    }
}
