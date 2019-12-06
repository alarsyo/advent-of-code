use std::collections::HashSet;
use std::fmt::Write;

use aoc::err;
use aoc::Result;

const INPUT: &str = include_str!("../input/day03.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;
    writeln!(res, "part 2: {}", part2(INPUT)?)?;

    Ok(res)
}

fn part1(input: &str) -> Result<usize> {
    let mut houses = HashSet::new();

    let mut x = 0;
    let mut y = 0;

    houses.insert((x, y));

    for c in input.trim_end().chars() {
        match c {
            '>' => x += 1,
            '<' => x -= 1,
            '^' => y -= 1,
            'v' => y += 1,
            _ => return Err(err!("unidentified move: `{}`", c)),
        }

        houses.insert((x, y));
    }

    Ok(houses.len())
}

fn part2(input: &str) -> Result<usize> {
    let mut houses = HashSet::new();

    let mut santa_x = 0;
    let mut santa_y = 0;

    let mut robot_x = 0;
    let mut robot_y = 0;

    houses.insert((0, 0));

    for (i, c) in input.trim_end().chars().enumerate() {
        if i % 2 == 0 {
            match c {
                '>' => santa_x += 1,
                '<' => santa_x -= 1,
                '^' => santa_y -= 1,
                'v' => santa_y += 1,
                _ => return Err(err!("unidentified move: `{}`", c)),
            }

            houses.insert((santa_x, santa_y));
        } else {
            match c {
                '>' => robot_x += 1,
                '<' => robot_x -= 1,
                '^' => robot_y -= 1,
                'v' => robot_y += 1,
                _ => return Err(err!("unidentified move: `{}`", c)),
            }

            houses.insert((robot_x, robot_y));
        }
    }

    Ok(houses.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_provided() {
        assert_eq!(part1(">").unwrap(), 2);
        assert_eq!(part1("^>v<").unwrap(), 4);
        assert_eq!(part1("^v^v^v^v^v").unwrap(), 2);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 2565);
    }

    #[test]
    fn part2_provided() {
        assert_eq!(part2("^v").unwrap(), 3);
        assert_eq!(part2("^>v<").unwrap(), 3);
        assert_eq!(part2("^v^v^v^v^v").unwrap(), 11);
    }
}
