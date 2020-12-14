use std::cmp::{max, min};
use std::fmt::Write;
use std::str::FromStr;

use anyhow::{bail, Context, Result};

const INPUT: &str = include_str!("../input/day03.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    let (first, second) = parse_wires(INPUT)?;

    writeln!(res, "part 1: {}", part1(&first, &second)?)?;
    writeln!(res, "part 2: {}", part2(&first, &second)?)?;

    Ok(res)
}

fn manhattan_distance(a: &Point, b: &Point) -> u64 {
    (a.x - b.x).abs() as u64 + (a.y - b.y).abs() as u64
}

fn part1(first_wire: &Wire, second_wire: &Wire) -> Result<u64> {
    first_wire
        .0
        .iter()
        .flat_map(|first_seg| {
            second_wire
                .0
                .iter()
                .map(move |second_seg| (first_seg, second_seg))
        })
        .filter_map(|(f, s)| match f.intersection(s) {
            Some(Point { x: 0, y: 0 }) | None => None,
            Some(p) => Some(p),
        })
        .map(|inter| manhattan_distance(&inter, &Point { x: 0, y: 0 }))
        .min()
        .context("wire was empty")
}

fn part2(first_wire: &Wire, second_wire: &Wire) -> Result<u64> {
    let mut min_dist = None;

    let mut first_length = 0;

    for seg1 in first_wire.0.iter() {
        let mut second_length = 0;

        for seg2 in second_wire.0.iter() {
            if let Some(inter) = seg1.intersection(&seg2) {
                if inter.x == 0 && inter.y == 0 {
                    continue;
                }
                let path_length = first_length
                    + second_length
                    + manhattan_distance(&inter, &seg1.begin)
                    + manhattan_distance(&inter, &seg2.begin);

                min_dist = match min_dist {
                    Some(dist) => Some(min(dist, path_length)),
                    None => Some(path_length),
                };
            }

            second_length += manhattan_distance(&seg2.begin, &seg2.end);
        }

        first_length += manhattan_distance(&seg1.begin, &seg1.end);
    }

    min_dist.context("wire was empty")
}

fn parse_wires(input: &str) -> Result<(Wire, Wire)> {
    let mut lines = input.lines();
    let first = lines.next().context("input missing a line")?.parse()?;
    let second = lines.next().context("input missing a line")?.parse()?;

    Ok((first, second))
}

#[derive(Debug)]
struct Wire(Vec<Segment>);

impl FromStr for Wire {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Wire> {
        let moves = s
            .trim_end()
            .split(',')
            .map(|m| m.parse().context("failed to parse wire"))
            .collect::<Result<Vec<Move>>>()?;

        let mut pos = Point { x: 0, y: 0 };

        let mut wire = Vec::with_capacity(moves.len());
        for mv in moves {
            let mut new_pos = pos;
            match mv.direction {
                Direction::Up => {
                    new_pos.y += mv.length;
                }
                Direction::Down => {
                    new_pos.y -= mv.length;
                }
                Direction::Left => {
                    new_pos.x -= mv.length;
                }
                Direction::Right => {
                    new_pos.x += mv.length;
                }
            }

            wire.push(Segment {
                begin: pos,
                end: new_pos,
                min_x: min(pos.x, new_pos.x),
                max_x: max(pos.x, new_pos.x),
                min_y: min(pos.y, new_pos.y),
                max_y: max(pos.y, new_pos.y),
            });

            pos = new_pos;
        }

        Ok(Wire(wire))
    }
}

#[derive(Clone, Copy, Debug)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Segment {
    begin: Point,
    end: Point,
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
}

impl Segment {
    fn intersection(&self, other: &Segment) -> Option<Point> {
        if self.min_x >= other.min_x
            && self.min_x <= other.max_x
            && other.min_y >= self.min_y
            && other.min_y <= self.max_y
        {
            Some(Point {
                x: self.min_x,
                y: other.min_y,
            })
        } else if other.min_x >= self.min_x
            && other.min_x <= self.max_x
            && self.min_y >= other.min_y
            && self.min_y <= other.max_y
        {
            Some(Point {
                x: other.min_x,
                y: self.min_y,
            })
        } else {
            None
        }
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Move {
    direction: Direction,
    length: i64,
}

impl FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let direction = s
            .chars()
            .nth(0)
            .context("couldn't get direction char in move")?;

        let s = s.get(1..).context("move missing length")?;

        let length = s.parse()?;

        let direction = match direction {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => bail!("couldn't parse direction: {}", direction),
        };

        Ok(Move { direction, length })
    }
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
        let (first, second) = parse_wires(PROVIDED1).unwrap();
        assert_eq!(part1(&first, &second).unwrap(), 6);
        let (first, second) = parse_wires(PROVIDED2).unwrap();
        assert_eq!(part1(&first, &second).unwrap(), 159);
        let (first, second) = parse_wires(PROVIDED3).unwrap();
        assert_eq!(part1(&first, &second).unwrap(), 135);
    }

    #[test]
    fn part1_real() {
        let (first, second) = parse_wires(INPUT).unwrap();
        assert_eq!(part1(&first, &second).unwrap(), 273);
    }

    #[test]
    fn part2_provided() {
        let (first, second) = parse_wires(PROVIDED1).unwrap();
        assert_eq!(part2(&first, &second).unwrap(), 30);
        let (first, second) = parse_wires(PROVIDED2).unwrap();
        assert_eq!(part2(&first, &second).unwrap(), 610);
        let (first, second) = parse_wires(PROVIDED3).unwrap();
        assert_eq!(part2(&first, &second).unwrap(), 410);
    }

    #[test]
    fn part2_real() {
        let (first, second) = parse_wires(INPUT).unwrap();
        assert_eq!(part2(&first, &second).unwrap(), 15622);
    }
}
