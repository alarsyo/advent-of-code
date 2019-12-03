use std::collections::HashSet;
use std::error::Error;
use std::str::FromStr;

use aoc::err;
use aoc::Result;

const INPUT: &str = include_str!("../input/day03.txt");

pub fn run() -> Result<()> {
    println!("part 1: {}", part1(INPUT)?);
    Ok(())
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

fn parse_path(line: &str) -> Result<Vec<Move>> {
    line.trim_end().split(',').map(|m| m.parse()).collect()
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

fn manhattan_distance(a: (i64, i64), b: (i64, i64)) -> i64 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn part1(input: &str) -> Result<i64> {
    let mut lines = input.lines();
    let first_path = parse_path(lines.next().ok_or_else(|| err!("missing line in input"))?)?;
    let second_path = parse_path(lines.next().ok_or_else(|| err!("missing line in input"))?)?;

    let mut first_x = 0;
    let mut first_y = 0;

    let mut second_x = 0;
    let mut second_y = 0;

    let mut first_locations = HashSet::new();
    let mut cross_locations = HashSet::new();

    for mv in first_path {
        let new_pos = match mv {
            Move::Up(dy) => (first_x, first_y - dy),
            Move::Down(dy) => (first_x, first_y + dy),
            Move::Left(dx) => (first_x - dx, first_y),
            Move::Right(dx) => (first_x + dx, first_y),
        };

        for cell in path((first_x, first_y), new_pos) {
            first_locations.insert(cell);
        }

        first_x = new_pos.0;
        first_y = new_pos.1;
    }

    for mv in second_path {
        let new_pos = match mv {
            Move::Up(dy) => (second_x, second_y - dy),
            Move::Down(dy) => (second_x, second_y + dy),
            Move::Left(dx) => (second_x - dx, second_y),
            Move::Right(dx) => (second_x + dx, second_y),
        };

        for cell in path((second_x, second_y), new_pos) {
            if first_locations.contains(&cell) {
                cross_locations.insert(cell);
            }
        }

        second_x = new_pos.0;
        second_y = new_pos.1;
    }

    cross_locations
        .iter()
        .map(|(x, y)| manhattan_distance((*x, *y), (0, 0)))
        .min()
        .ok_or_else(|| err!("the cables never crossed !"))
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
}
