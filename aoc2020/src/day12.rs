use std::fmt::Write;

use aoc::err;

const INPUT: &str = include_str!("../input/day12.txt");

pub fn run() -> aoc::Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;
    writeln!(res, "part 2: {}", part2(INPUT)?)?;

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

fn part2(input: &str) -> aoc::Result<i64> {
    let actions: Vec<Action> = input
        .lines()
        .map(|line| line.parse())
        .collect::<aoc::Result<_>>()?;

    let mut ship = Ship::new();

    for a in actions {
        ship.process_with_waypoint(a);
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TurnDirection {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ActionKind {
    Move(Direction),

    Turn(TurnDirection),

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

            'L' => ActionKind::Turn(TurnDirection::Left),
            'R' => ActionKind::Turn(TurnDirection::Right),

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

    waypoint: Waypoint,
}

impl Ship {
    fn new() -> Self {
        Self {
            direction: Direction::East,
            x: 0,
            y: 0,
            waypoint: Waypoint::new(),
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

    fn find_direction<I>(iter: I, quarters: usize, current_direction: Direction) -> Direction
    where
        I: Iterator<Item = Direction>,
        I: std::clone::Clone,
    {
        iter
            // this is litteraly a circle, reaching West and turning 90 degrees right means
            // facing North again
            .cycle()
            // find our current ship direction
            .skip_while(|dir| *dir != current_direction)
            // skip as many quarters as needed
            .nth(quarters)
            // we can unwrap safely because we called .cycle() on a non empty iterator
            .unwrap()
    }

    fn process(&mut self, action: Action) {
        match action.kind {
            ActionKind::Move(dir) => self.forward(dir, action.arg),

            ActionKind::Turn(turn_dir) => {
                debug_assert!(action.arg % 90 == 0, "only right angles are supported");

                let quarters = (action.arg / 90) as usize;

                let directions_iter = Self::CLOCKWISE_DIRECTIONS.iter().copied();

                let new_dir = if turn_dir == TurnDirection::Left {
                    // go through cardinal directions the other way around, anti-clockwise
                    Ship::find_direction(directions_iter.rev(), quarters, self.direction)
                } else {
                    Ship::find_direction(directions_iter, quarters, self.direction)
                };

                self.direction = new_dir;
            }

            ActionKind::Forward => self.forward(self.direction, action.arg),
        }
    }

    fn process_with_waypoint(&mut self, action: Action) {
        match action.kind {
            ActionKind::Move(dir) => match dir {
                Direction::North => self.waypoint.y -= action.arg as i64,
                Direction::South => self.waypoint.y += action.arg as i64,
                Direction::West => self.waypoint.x -= action.arg as i64,
                Direction::East => self.waypoint.x += action.arg as i64,
            },

            ActionKind::Turn(turn_dir) => {
                debug_assert!(action.arg % 90 == 0, "only right angles are supported");

                let quadrants = (action.arg / 90) as usize % 4;

                self.waypoint.turn(turn_dir, quadrants);
            }

            ActionKind::Forward => {
                let (west_east, north_south) = self.waypoint.as_dirs();

                self.forward(west_east, self.waypoint.x.abs() as u16 * action.arg);
                self.forward(north_south, self.waypoint.y.abs() as u16 * action.arg);
            }
        }
    }

    fn forward(&mut self, direction: Direction, arg: u16) {
        let arg = arg as i64;

        match direction {
            Direction::North => self.y -= arg as i64,
            Direction::South => self.y += arg as i64,
            Direction::West => self.x -= arg as i64,
            Direction::East => self.x += arg as i64,
        }
    }
}

#[derive(Debug, Clone)]
struct Waypoint {
    x: i64,
    y: i64,
}

impl Waypoint {
    fn new() -> Self {
        Self { x: 10, y: -1 }
    }

    fn as_dirs(&self) -> (Direction, Direction) {
        let west_east = if self.x < 0 {
            Direction::West
        } else {
            Direction::East
        };
        let north_south = if self.y < 0 {
            Direction::North
        } else {
            Direction::South
        };

        (west_east, north_south)
    }

    fn turn(&mut self, turn_dir: TurnDirection, quadrants: usize) {
        for _ in 0..quadrants {
            let mut x = self.x;
            let mut y = self.y;

            match turn_dir {
                TurnDirection::Left => x = -x,
                TurnDirection::Right => y = -y,
            }

            self.x = y;
            self.y = x;
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

    #[test]
    fn part2_provided() {
        assert_eq!(part2(PROVIDED).unwrap(), 286);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT).unwrap(), 23960);
    }
}
