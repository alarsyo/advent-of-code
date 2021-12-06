use std::fmt::Write;
use std::str;

use anyhow::Result;

const INPUT: &str = include_str!("../input/day06.txt");

const SPAWNING_DELAY: usize = 7;
const TURNS_PART_1: usize = 80;
const TURNS_PART_2: usize = 256;

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;
    writeln!(res, "part 2: {}", part2(INPUT)?)?;

    Ok(res)
}

fn part1(input: &str) -> Result<usize> {
    let mut school = input.trim().parse::<School>()?;

    Ok(school.grow_for(TURNS_PART_1))
}

fn part2(input: &str) -> Result<usize> {
    let mut school = input.trim().parse::<School>()?;

    Ok(school.grow_for(TURNS_PART_2))
}

struct School {
    fish_timers: [usize; SPAWNING_DELAY as usize + 2],
}

impl School {
    fn next_turn(&mut self) {
        let newly_spawned = self.fish_timers[0];

        for i in 1..self.fish_timers.len() {
            self.fish_timers[i - 1] = self.fish_timers[i];
        }

        self.fish_timers[SPAWNING_DELAY - 1] += newly_spawned;
        *self.fish_timers.last_mut().unwrap() = newly_spawned;
    }

    fn size(&self) -> usize {
        self.fish_timers.iter().sum()
    }

    fn grow_for(&mut self, turns: usize) -> usize {
        for _ in 0..turns {
            self.next_turn();
        }

        self.size()
    }
}

impl std::str::FromStr for School {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut fish_timers = [0usize; SPAWNING_DELAY + 2];

        for fish in s.split(',').map(str::parse::<usize>) {
            fish_timers[fish?] += 1;
        }

        Ok(School { fish_timers })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED: &str = include_str!("../input/day06_provided.txt");

    #[test]
    fn part1_provided() {
        assert_eq!(part1(PROVIDED).unwrap(), 5934);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 350149);
    }

    #[test]
    fn part2_provided() {
        assert_eq!(part2(PROVIDED).unwrap(), 26984457539);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT).unwrap(), 1590327954513);
    }
}
