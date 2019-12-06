use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fmt::Write;
use std::str::FromStr;

use aoc::err;
use aoc::Result;

const INPUT: &str = include_str!("../input/day03.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;
    writeln!(res, "part 2: {}", part2(INPUT)?)?;

    Ok(res)
}

enum Move {
    Up(i64),
    Down(i64),
    Left(i64),
    Right(i64),
}

impl FromStr for Move {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        let direction = s
            .chars()
            .nth(0)
            .ok_or_else(|| err!("couldn't get direction char in move: {}", s))?;

        let s = s
            .get(1..)
            .ok_or_else(|| err!("move missing length: {}", s))?;

        let length = s.parse()?;

        match direction {
            'U' => Ok(Move::Up(length)),
            'D' => Ok(Move::Down(length)),
            'L' => Ok(Move::Left(length)),
            'R' => Ok(Move::Right(length)),
            _ => Err(err!("couldn't parse direction: {}", direction)),
        }
    }
}

fn path(mut a: (i64, i64), b: (i64, i64)) -> Vec<(i64, i64)> {
    let mut res = Vec::new();

    while a != b {
        if a.0 < b.0 {
            a.0 += 1;
        } else if a.0 > b.0 {
            a.0 -= 1;
        }

        if a.1 < b.1 {
            a.1 += 1;
        } else if a.1 > b.1 {
            a.1 -= 1;
        }

        res.push(a);
    }

    res
}

fn parse_path(line: &str) -> Result<HashSet<(i64, i64)>> {
    let moves = line
        .trim_end()
        .split(',')
        .map(|m| m.parse())
        .collect::<Result<Vec<Move>>>()?;

    let mut pos = (0, 0);

    Ok(moves
        .iter()
        .flat_map(|mv| {
            let new_pos = match mv {
                Move::Up(dy) => (pos.0, pos.1 - dy),
                Move::Down(dy) => (pos.0, pos.1 + dy),
                Move::Left(dx) => (pos.0 - dx, pos.1),
                Move::Right(dx) => (pos.0 + dx, pos.1),
            };

            let p = path(pos, new_pos);
            pos = new_pos;
            p
        })
        .collect())
}

fn manhattan_distance(a: (i64, i64), b: (i64, i64)) -> i64 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn part1(input: &str) -> Result<i64> {
    let mut lines = input.lines();
    let first_path = parse_path(lines.next().ok_or_else(|| err!("missing line in input"))?)?;
    let second_path = parse_path(lines.next().ok_or_else(|| err!("missing line in input"))?)?;

    let cross_locations = first_path.intersection(&second_path);

    cross_locations
        .map(|(x, y)| manhattan_distance((*x, *y), (0, 0)))
        .min()
        .ok_or_else(|| err!("the cables never crossed !"))
}

fn path_count(mut a: (i64, i64), b: (i64, i64), mut start_count: i64) -> Vec<((i64, i64), i64)> {
    let mut res = Vec::new();

    while a != b {
        if a.0 < b.0 {
            a.0 += 1;
        } else if a.0 > b.0 {
            a.0 -= 1;
        }

        if a.1 < b.1 {
            a.1 += 1;
        } else if a.1 > b.1 {
            a.1 -= 1;
        }

        start_count += 1;
        res.push((a, start_count));
    }

    res
}

fn parse_path_with_count(line: &str) -> Result<HashMap<(i64, i64), i64>> {
    let moves = line
        .trim_end()
        .split(',')
        .map(|m| m.parse())
        .collect::<Result<Vec<Move>>>()?;

    let mut pos = (0, 0);
    let mut count = 0;

    Ok(moves
        .iter()
        .flat_map(|mv| {
            let new_pos = match mv {
                Move::Up(dy) => (pos.0, pos.1 - dy),
                Move::Down(dy) => (pos.0, pos.1 + dy),
                Move::Left(dx) => (pos.0 - dx, pos.1),
                Move::Right(dx) => (pos.0 + dx, pos.1),
            };

            let p = path_count(pos, new_pos, count);
            count += manhattan_distance(pos, new_pos);
            pos = new_pos;
            p
        })
        .collect())
}

fn part2(input: &str) -> Result<i64> {
    let mut lines = input.lines();
    let first_path =
        parse_path_with_count(lines.next().ok_or_else(|| err!("missing line in input"))?)?;
    let second_path =
        parse_path_with_count(lines.next().ok_or_else(|| err!("missing line in input"))?)?;

    first_path
        .keys()
        .filter(|pos| second_path.contains_key(&pos))
        .map(|pos| first_path.get(&pos).unwrap() + second_path.get(&pos).unwrap())
        .min()
        .ok_or_else(|| err!("cables never crossed !"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED1: &str = "R8,U5,L5,D3
U7,R6,D4,L4
";

    const PROVIDED2: &str = "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83
";

    const PROVIDED3: &str = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7
";

    #[test]
    fn part1_provided() {
        assert_eq!(part1(PROVIDED1).unwrap(), 6);
        assert_eq!(part1(PROVIDED2).unwrap(), 159);
        assert_eq!(part1(PROVIDED3).unwrap(), 135);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 273);
    }

    #[test]
    fn part2_provided() {
        assert_eq!(part2(PROVIDED1).unwrap(), 30);
        assert_eq!(part2(PROVIDED2).unwrap(), 610);
        assert_eq!(part2(PROVIDED3).unwrap(), 410);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT).unwrap(), 15622);
    }
}
