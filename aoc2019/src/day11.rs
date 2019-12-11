use std::collections::HashMap;
use std::fmt::Write;

use aoc::err;
use aoc::Result;

use crate::intcode::Intcode;

const INPUT: &str = include_str!("../input/day11.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;
    writeln!(res, "part 2:")?;
    part2(INPUT, &mut res)?;

    Ok(res)
}

fn part1(input: &str) -> Result<usize> {
    let mut robot = Robot::new(input)?;
    let mut board = HashMap::new();

    robot.run(&mut board)?;

    Ok(board.len())
}

fn part2(input: &str, res: &mut String) -> Result<()> {
    let mut robot = Robot::new(input)?;
    let mut board = HashMap::new();

    board.insert(robot.pos, true);

    robot.run(&mut board)?;

    write_board(res, board)
}

fn write_board(res: &mut String, board: HashMap<Position, bool>) -> Result<()> {
    if board.is_empty() {
        return Err(err!("board was empty"));
    }

    let min_x = board.keys().map(|p| p.x).min().unwrap();
    let max_x = board.keys().map(|p| p.x).max().unwrap();
    let min_y = board.keys().map(|p| p.y).min().unwrap();
    let max_y = board.keys().map(|p| p.y).max().unwrap();

    let width = max_x - min_x + 1;
    let height = max_y - min_y + 1;

    for i in 0..height {
        for j in 0..width {
            match board.get(&Position {
                x: j + min_x,
                y: i + min_y,
            }) {
                Some(true) => write!(res, "█")?,
                _ => write!(res, " ")?,
            };
        }
        writeln!(res)?;
    }

    Ok(())
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Position {
    x: i64,
    y: i64,
}

struct Robot {
    pos: Position,
    dir: Direction,
    brain: Intcode,
}

impl Robot {
    fn new(program: &str) -> Result<Self> {
        let intcode = Intcode::new(program)?;

        Ok(Robot {
            pos: Position { x: 0, y: 0 },
            dir: Direction::Up,
            brain: intcode,
        })
    }

    fn run(&mut self, board: &mut HashMap<Position, bool>) -> Result<()> {
        while !self.brain.run_and_wait()? {
            if !self.brain.output.is_empty() {
                let color = self.brain.output[0];
                let direction = self.brain.output[1];

                match color {
                    0 => board.insert(self.pos, false),
                    1 => board.insert(self.pos, true),
                    _ => return Err(err!("robot brain output different from 0 or 1")),
                };

                match direction {
                    0 => self.turn_left(),
                    1 => self.turn_right(),
                    _ => return Err(err!("robot brain output different from 0 or 1")),
                };
                self.move_forward();
                self.brain.output.clear();
            }

            let paint = match board.get(&self.pos) {
                Some(true) => 1,
                _ => 0,
            };

            self.brain.add_input(paint);
        }

        Ok(())
    }

    fn move_forward(&mut self) {
        match self.dir {
            Direction::Up => self.pos.y -= 1,
            Direction::Down => self.pos.y += 1,
            Direction::Left => self.pos.x -= 1,
            Direction::Right => self.pos.x += 1,
        }
    }

    fn turn_right(&mut self) {
        match self.dir {
            Direction::Up => self.dir = Direction::Right,
            Direction::Right => self.dir = Direction::Down,
            Direction::Down => self.dir = Direction::Left,
            Direction::Left => self.dir = Direction::Up,
        }
    }

    fn turn_left(&mut self) {
        match self.dir {
            Direction::Up => self.dir = Direction::Left,
            Direction::Right => self.dir = Direction::Up,
            Direction::Down => self.dir = Direction::Right,
            Direction::Left => self.dir = Direction::Down,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const RES2: &str = include_str!("../input/day11_res.txt");

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 1883);
    }

    #[test]
    fn part2_real() {
        let mut res = String::with_capacity(RES2.len());

        part2(INPUT, &mut res).unwrap();
        assert_eq!(res.len(), RES2.len());
        assert_eq!(res, RES2);
    }
}
