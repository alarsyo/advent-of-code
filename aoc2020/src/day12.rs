use std::fmt::Write;

use aoc::err;

const INPUT: &str = include_str!("../input/day12.txt");

pub fn run() -> aoc::Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;

    Ok(res)
}

fn part1(input: &str) -> aoc::Result<i64> {
    let actions: Vec<Action> = input
        .lines()
        .map(|line| line.parse())
        .collect::<aoc::Result<_>>()?;

    let mut ship = Ship::new();

    for a in actions {
        ship.process(a);
    }

    Ok(ship.manhattan_distance())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Copy)]
enum ActionKind {
    Move(Direction),

    Left,
    Right,

    Forward,
}

#[derive(Debug, Clone)]
struct Action {
    kind: ActionKind,
    arg: u16,
}

impl std::str::FromStr for Action {
    type Err = aoc::Error;

    fn from_str(s: &str) -> aoc::Result<Self> {
        debug_assert!(
            s.len() >= 2,
            "tried to parse action but it is too short: `{}`",
            s
        );
        let letter = s
            .chars()
            .next()
            .ok_or_else(|| err!("couldn't parse action: empty string"))?;

        let kind = match letter {
            'N' => ActionKind::Move(Direction::North),
            'S' => ActionKind::Move(Direction::South),
            'E' => ActionKind::Move(Direction::East),
            'W' => ActionKind::Move(Direction::West),

            'L' => ActionKind::Left,
            'R' => ActionKind::Right,

            'F' => ActionKind::Forward,

            _ => return Err(err!("couldn't parse action with letter `{}`", letter)),
        };

        let arg = s[1..]
            .parse()
            .map_err(|e| err!("couldn't parse action arg: {}", e))?;

        Ok(Self { kind, arg })
    }
}

#[derive(Debug, Clone)]
struct Ship {
    direction: Direction,
    x: i64,
    y: i64,
}

impl Ship {
    fn new() -> Self {
        Self {
            direction: Direction::East,
            x: 0,
            y: 0,
        }
    }

    const CLOCKWISE_DIRECTIONS: &'static [Direction] = &[
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];

    fn manhattan_distance(&self) -> i64 {
        self.x.abs() + self.y.abs()
    }

    fn process(&mut self, action: Action) {
        match action.kind {
            ActionKind::Move(dir) => self.forward(dir, action.arg),

            ActionKind::Right => {
                debug_assert!(action.arg % 90 == 0, "only right angles are supported");

                let quarters = action.arg / 90;

                let new_dir = Self::CLOCKWISE_DIRECTIONS
                    .iter()
                    // this is litteraly a circle, reaching West and turning 90 degrees right means
                    // facing North again
                    .cycle()
                    // find our current ship direction
                    .skip_while(|dir| **dir != self.direction)
                    // skip as many quarters as needed
                    .nth(quarters as usize)
                    // we can unwrap safely because we called .cycle() on a non empty iterator
                    .unwrap();

                self.direction = *new_dir;
            }

            ActionKind::Left => {
                debug_assert!(action.arg % 90 == 0, "only right angles are supported");

                let quarters = action.arg / 90;

                let new_dir = Self::CLOCKWISE_DIRECTIONS
                    .iter()
                    // same thing as above, but reverse direction first!
                    .rev()
                    .cycle()
                    .skip_while(|dir| **dir != self.direction)
                    .nth(quarters as usize)
                    .unwrap();

                self.direction = *new_dir;
            }

            ActionKind::Forward => self.forward(self.direction, action.arg),
        }
    }

    fn forward(&mut self, direction: Direction, arg: u16) {
        let arg = arg as i64;

        match direction {
            Direction::North => self.y -= arg,
            Direction::South => self.y += arg,
            Direction::West => self.x -= arg,
            Direction::East => self.x += arg,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED: &str = include_str!("../input/day12_provided.txt");

    #[test]
    fn part1_provided() {
        assert_eq!(part1(PROVIDED).unwrap(), 25);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 1589);
    }
}
