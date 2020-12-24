use std::{collections::VecDeque, fmt::Write};

use anyhow::{Context, Result};

const INPUT: &str = include_str!("../input/day23.txt");

const CUP_NUMBER: usize = 1_000_000;
const TURNS_NUMBER: usize = 10_000_000;

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;
    writeln!(res, "part 2: {}", part2(INPUT)?)?;

    Ok(res)
}

fn part1(input: &str) -> Result<String> {
    let mut cup_circle: CupCircle = input.parse()?;

    for _ in 0..100 {
        cup_circle.step();
    }

    Ok(format!("{}", cup_circle))
}

fn part2(input: &str) -> Result<usize> {
    let mut cup_circle: FastCupCircle = input.parse()?;

    for _ in 0..TURNS_NUMBER {
        cup_circle.step();
    }

    let first = *cup_circle.next_cup(1);
    let second = *cup_circle.next_cup(first);

    Ok(first * second)
}

/// CupCircle provides an abstraction over a VecDeque to emulate the steps of the game
struct CupCircle(VecDeque<u64>);

/// The "current" cup should always be the first one in the VecDeque at the end of a turn, for
/// convenience
///
/// The Circle should never become empty if you only use its public interface, so all calls to
/// `unwrap()` in its implementation should never panic.
impl CupCircle {
    /// Shifts the cup circle, putting the first cup at the end of the deque
    ///
    /// This doesn't change anything to the cup circle layout, except that the new first cup is
    /// considered the "current" cup at the end of a step in our implementation
    fn shift(&mut self) {
        let front = self.0.pop_front().unwrap();
        self.0.push_back(front);
    }

    /// Executes one step of the game
    pub fn step(&mut self) {
        let current = self.front();
        // skip current
        self.shift();

        // The crab picks up the three cups that are immediately clockwise of the current cup. They
        // are removed from the circle; cup spacing is adjusted as necessary to maintain the circle.
        let removed_cups = self.remove_next_3();

        // The crab selects a destination cup: the cup with a label equal to the current cup's label
        // minus one. If this would select one of the cups that was just picked up, the crab will
        // keep subtracting one until it finds a cup that wasn't just picked up. If at any point in
        // this process the value goes below the lowest value on any cup's label, it wraps around to
        // the highest value on any cup's label instead.
        //
        // TODO: use std::cmp::Ord::clamp when stabilized (Rust 1.50)
        let mut destination = if current > 1 { current - 1 } else { self.max() };
        while removed_cups.contains(&destination) {
            destination = if destination > 1 {
                destination - 1
            } else {
                self.max()
            };
        }

        // place destination in front
        while self.front() != destination {
            self.shift();
        }

        // The crab places the cups it just picked up so that they are immediately clockwise of the
        // destination cup. They keep the same order as when they were picked up.
        //
        // For this, let's put the destination at the end of the queue by shifting one last time
        self.shift();
        removed_cups.iter().for_each(|cup| self.push_back(*cup));

        // The crab selects a new current cup: the cup which is immediately clockwise of the current
        // cup.
        while self.front() != current {
            self.shift();
        }
        self.shift();
    }

    fn remove_next_3(&mut self) -> [u64; 3] {
        let first = self.pop_front();
        let second = self.pop_front();
        let third = self.pop_front();

        [first, second, third]
    }

    pub fn front(&self) -> u64 {
        *self.0.front().unwrap()
    }

    fn pop_front(&mut self) -> u64 {
        self.0.pop_front().unwrap()
    }

    fn push_back(&mut self, value: u64) {
        self.0.push_back(value);
    }

    pub fn max(&self) -> u64 {
        *self.0.iter().max().unwrap()
    }
}

impl std::str::FromStr for CupCircle {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let circle = s
            .trim_end()
            .chars()
            .map(|c| Ok(c.to_digit(10).context("character was not a digit")? as u64))
            .collect::<Result<_>>()?;

        Ok(CupCircle(circle))
    }
}

impl std::fmt::Display for CupCircle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0
            .iter()
            .cycle()
            .skip_while(|cup| **cup != 1)
            .skip(1)
            .take(8)
            .try_for_each(|cup| write!(f, "{}", cup))
    }
}

/// CupCircle provides a fast abstraction to emulate the steps of the game.
///
/// It is considerably faster than the above naive implementation, but its representation isn't as
/// intuitive. It uses a [`std::vec::Vec`] of indices, where `vec[cup]` returns the next cup in the
/// circle, for a given cup.
struct FastCupCircle {
    cups: Vec<usize>,
    current: usize,
}

impl FastCupCircle {
    fn next_cup(&self, cup: usize) -> &usize {
        &self.cups[cup - 1]
    }

    fn next_cup_mut(&mut self, cup: usize) -> &mut usize {
        &mut self.cups[cup - 1]
    }

    fn remove_next_3(&mut self, cup: usize) -> [usize; 3] {
        let first = *self.next_cup(cup);
        let second = *self.next_cup(first);
        let third = *self.next_cup(second);

        // shortcut the links to remove them from the loop temporarily
        *self.next_cup_mut(cup) = *self.next_cup(third);

        [first, second, third]
    }

    fn step(&mut self) {
        // The crab picks up the three cups that are immediately clockwise of the current cup. They
        // are removed from the circle; cup spacing is adjusted as necessary to maintain the circle.
        let removed_cups = self.remove_next_3(self.current);

        // The crab selects a destination cup: the cup with a label equal to the current cup's label
        // minus one. If this would select one of the cups that was just picked up, the crab will
        // keep subtracting one until it finds a cup that wasn't just picked up. If at any point in
        // this process the value goes below the lowest value on any cup's label, it wraps around to
        // the highest value on any cup's label instead.
        //
        // TODO: use std::cmp::Ord::clamp when stabilized (Rust 1.50)
        let mut destination = if self.current > 1 {
            self.current - 1
        } else {
            self.cups.len()
        };
        while removed_cups.contains(&destination) {
            destination = if destination > 1 {
                destination - 1
            } else {
                self.cups.len()
            };
        }

        // The crab places the cups it just picked up so that they are immediately clockwise of the
        // destination cup. They keep the same order as when they were picked up.
        //
        // The links from first to second and from second to third haven't changed, no need to
        // update them
        let [first, _, third] = removed_cups;
        *self.next_cup_mut(third) = *self.next_cup(destination);
        *self.next_cup_mut(destination) = first;

        // The crab selects a new current cup: the cup which is immediately clockwise of the current
        // cup.
        self.current = *self.next_cup(self.current);
    }
}

impl std::str::FromStr for FastCupCircle {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let input_cups: Vec<usize> = s
            .trim_end()
            .chars()
            .map(|c| Ok(c.to_digit(10).context("character was not a digit")? as usize))
            .collect::<Result<_>>()?;

        let max = *input_cups.iter().max().context("input contained 0 cups")?;

        let mut cups = Vec::new();
        cups.resize_with(CUP_NUMBER, Default::default);

        let cup_iter = input_cups.clone().into_iter().chain((max + 1)..=CUP_NUMBER);
        let next_cup_iter = input_cups
            .into_iter()
            .chain((max + 1)..=CUP_NUMBER)
            .cycle()
            .skip(1);

        for (cup, next) in cup_iter.zip(next_cup_iter) {
            cups[cup - 1] = next;
        }

        let current = cups[CUP_NUMBER - 1];

        Ok(Self { cups, current })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED: &str = include_str!("../input/day23_provided.txt");

    #[test]
    fn part1_provided() {
        assert_eq!(part1(PROVIDED).unwrap(), "67384529");
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), "72496583");
    }

    #[test]
    #[ignore]
    fn part2_provided() {
        assert_eq!(part2(PROVIDED).unwrap(), 149245887792);
    }

    #[test]
    #[ignore]
    fn part2_real() {
        assert_eq!(part2(INPUT).unwrap(), 41785843847);
    }
}
