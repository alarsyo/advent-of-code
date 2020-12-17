use std::collections::HashSet;
use std::fmt::Write;

use anyhow::{Context, Result};

const INPUT: &str = include_str!("../input/day10.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;

    Ok(res)
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Position {
    x: i64,
    y: i64,
}

#[derive(Debug, PartialEq)]
struct Asteroid {
    pos: Position,
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn part1(input: &str) -> Result<usize> {
    let mut asteroids = Vec::new();
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                asteroids.push(Asteroid {
                    pos: Position {
                        x: j as i64,
                        y: i as i64,
                    },
                })
            }
        }
    }

    let mut best = None;
    for a in &asteroids {
        let mut set = HashSet::new();

        for b in &asteroids {
            if a == b {
                continue;
            }

            let direction = Position {
                x: b.pos.x - a.pos.x,
                y: b.pos.y - a.pos.y,
            };

            let mut div = gcd(direction.x, direction.y);
            if div < 0 {
                div *= -1;
            }

            set.insert(Position {
                x: direction.x / div,
                y: direction.y / div,
            });
        }

        best = match best {
            None => Some(set),
            Some(old) => {
                if set.len() > old.len() {
                    Some(set)
                } else {
                    Some(old)
                }
            }
        };
    }

    let best = best.context("zero asteroid provided")?;
    Ok(best.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED1: &str = ".#..#
.....
#####
....#
...##
";

    const PROVIDED2: &str = "......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####
";

    const PROVIDED3: &str = "#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.
";

    const PROVIDED4: &str = ".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..
";

    const PROVIDED5: &str = ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##
";

    #[test]
    fn part1_provided() {
        assert_eq!(part1(PROVIDED1).unwrap(), 8);
        assert_eq!(part1(PROVIDED2).unwrap(), 33);
        assert_eq!(part1(PROVIDED3).unwrap(), 35);
        assert_eq!(part1(PROVIDED4).unwrap(), 41);
        assert_eq!(part1(PROVIDED5).unwrap(), 210);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 214);
    }
}
