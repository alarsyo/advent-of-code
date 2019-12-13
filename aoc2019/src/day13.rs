use std::collections::HashMap;
use std::fmt::{self, Display, Write};

use aoc::err;
use aoc::Result;

use crate::intcode::{parse_memory, Intcode};

const INPUT: &str = include_str!("../input/day13.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    let memory = parse_memory(INPUT)?;
    writeln!(res, "part 1: {}", part1(memory.clone())?)?;
    writeln!(res, "part 2: {}", part2(memory)?)?;

    Ok(res)
}

fn part1(memory: Vec<i64>) -> Result<usize> {
    let mut intcode = Intcode::with_memory(memory);
    intcode.run()?;

    let mut map = HashMap::new();

    for c in intcode.output.chunks(3) {
        let pos = (c[0], c[1]);
        let tile = Tile::new(c[2])?;

        map.insert(pos, tile);
    }

    Ok(map
        .values()
        .filter(|t| match t {
            Tile::Block => true,
            _ => false,
        })
        .count())
}

#[allow(dead_code)]
fn print_screen(screen: &[Vec<Tile>]) {
    for line in screen {
        for tile in line {
            print!("{}", tile);
        }
        println!();
    }
}

fn get_next_move(paddle_pos: (i64, i64), ball_pos: (i64, i64)) -> i64 {
    if ball_pos.0 > paddle_pos.0 {
        1
    } else if ball_pos.0 == paddle_pos.0 {
        0
    } else {
        -1
    }
}

fn part2(mut memory: Vec<i64>) -> Result<i64> {
    // put coin in
    memory[0] = 2;

    let mut intcode = Intcode::with_memory(memory);

    let mut score = 0;
    let mut paddle_pos = (0, 0);
    let mut ball_pos = (0, 0);

    loop {
        let halted = intcode.run_and_wait()?;

        for c in intcode.output.chunks(3) {
            let pos = (c[0], c[1]);
            match pos {
                (-1, 0) => {
                    score = c[2];
                }
                _ => {
                    let tile = Tile::new(c[2])?;
                    match tile {
                        Tile::Paddle => paddle_pos = pos,
                        Tile::Ball => ball_pos = pos,
                        _ => {}
                    }
                }
            };
        }
        intcode.output.clear();

        if halted {
            break;
        }

        let mv = get_next_move(paddle_pos, ball_pos);
        intcode.add_input(mv);
    }

    Ok(score)
}

#[derive(Clone, Copy)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl Tile {
    fn new(n: i64) -> Result<Self> {
        let tile = match n {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::Paddle,
            4 => Tile::Ball,
            _ => return Err(err!("couldn't associate number with tile: {}", n)),
        };

        Ok(tile)
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Tile::Empty => write!(f, " "),
            Tile::Wall => write!(f, "|"),
            Tile::Block => write!(f, "█"),
            Tile::Paddle => write!(f, "_"),
            Tile::Ball => write!(f, "O"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_real() {
        let memory = parse_memory(INPUT).unwrap();
        assert_eq!(part1(memory).unwrap(), 298);
    }

    #[test]
    fn part2_real() {
        let memory = parse_memory(INPUT).unwrap();
        assert_eq!(part2(memory).unwrap(), 13956);
    }
}
