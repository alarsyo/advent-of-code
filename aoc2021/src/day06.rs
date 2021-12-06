use std::fmt::Write;
use std::str;

use anyhow::Result;

const INPUT: &str = include_str!("../input/day06.txt");

const SPAWNING_DELAY: u8 = 7;
const TURNS_PART_1: usize = 80;
const TURNS_PART_2: usize = 256;

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;
    writeln!(res, "part 2: {}", part2(INPUT)?)?;

    Ok(res)
}

fn part1(input: &str) -> Result<usize> {
    let mut school = input
        .trim()
        .split(',')
        .map(str::parse::<LanternFish>)
        .collect::<Result<Vec<_>>>()?;

    for _ in 0..TURNS_PART_1 {
        let mut newly_spawned = 0;

        for fish in &mut school {
            if fish.next_turn() {
                newly_spawned += 1;
            }
        }

        school.resize_with(school.len() + newly_spawned, LanternFish::default)
    }

    Ok(school.len())
}

fn part2(input: &str) -> Result<usize> {
    let mut school = input.trim().parse::<School>()?;

    for _ in 0..TURNS_PART_2 {
        school.next_turn();
    }

    Ok(school.size())
}

struct LanternFish {
    timer: u8,
}

impl LanternFish {
    fn spawn() -> Self {
        LanternFish {
            timer: SPAWNING_DELAY + 1,
        }
    }

    fn next_turn(&mut self) -> bool {
        if self.timer == 0 {
            self.timer = SPAWNING_DELAY - 1;
            true
        } else {
            self.timer -= 1;
            false
        }
    }
}

impl Default for LanternFish {
    fn default() -> Self {
        Self::spawn()
    }
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

        self.fish_timers[SPAWNING_DELAY as usize - 1] += newly_spawned;
        *self.fish_timers.last_mut().unwrap() = newly_spawned;
    }

    fn size(&self) -> usize {
        self.fish_timers.iter().sum()
    }
}

impl std::str::FromStr for School {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut fish_timers = [0usize; SPAWNING_DELAY as usize + 2];

        for fish in s.split(',').map(str::parse::<usize>) {
            let fish = fish?;
            fish_timers[fish] += 1;
        }

        Ok(School { fish_timers })
    }
}

impl std::str::FromStr for LanternFish {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(LanternFish { timer: s.parse()? })
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
