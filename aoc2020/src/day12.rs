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
        ship.process(&a);
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
        ship.process_with_waypoint(&a);
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

impl Direction {
    const CLOCKWISE_DIRECTIONS: &'static [Direction] = &[
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];

    fn rotate(self, turn_dir: TurnDirection, degrees: i64) -> Direction {
        debug_assert!(degrees % 90 == 0, "only right angles are supported");
        let quadrants = (degrees / 90) as usize;

        let directions_iter = Self::CLOCKWISE_DIRECTIONS.iter().copied();

        if turn_dir == TurnDirection::Left {
            // go through cardinal directions the other way around, anti-clockwise
            Self::find_direction(directions_iter.rev(), quadrants, self)
        } else {
            Self::find_direction(directions_iter, quadrants, self)
        }
    }

    fn find_direction<I>(iter: I, quarters: usize, current_direction: Direction) -> Direction
    where
        I: Iterator<Item = Direction>,
        I: std::clone::Clone,
    {
        iter
            // this is litteraly a circle, reaching West and turning 90 degrees right means facing
            // North again
            .cycle()
            // find our current ship direction
            .skip_while(|dir| *dir != current_direction)
            // skip as many quarters as needed
            .nth(quarters)
            // we can unwrap safely because we called .cycle() on a non empty iterator
            .unwrap()
    }
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
    arg: i64,
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
struct Coordinates {
    x: i64,
    y: i64,
}

impl Coordinates {
    fn move_towards(&mut self, direction: Direction, distance: i64) {
        match direction {
            Direction::North => self.y -= distance,
            Direction::South => self.y += distance,
            Direction::West => self.x -= distance,
            Direction::East => self.x += distance,
        }
    }
}

#[derive(Debug, Clone)]
struct Ship {
    direction: Direction,
    coordinates: Coordinates,

    waypoint: Waypoint,
}

impl Ship {
    fn new() -> Self {
        Self {
            direction: Direction::East,
            coordinates: Coordinates { x: 0, y: 0 },
            waypoint: Waypoint::new(),
        }
    }

    fn manhattan_distance(&self) -> i64 {
        self.coordinates.x.abs() + self.coordinates.y.abs()
    }

    fn process(&mut self, action: &Action) {
        match action.kind {
            ActionKind::Move(dir) => self.coordinates.move_towards(dir, action.arg),

            ActionKind::Turn(turn_dir) => {
                self.direction = self.direction.rotate(turn_dir, action.arg);
            }

            ActionKind::Forward => self.coordinates.move_towards(self.direction, action.arg),
        }
    }

    fn process_with_waypoint(&mut self, action: &Action) {
        match action.kind {
            ActionKind::Move(dir) => self.waypoint.coordinates.move_towards(dir, action.arg),

            ActionKind::Turn(turn_dir) => {
                debug_assert!(action.arg % 90 == 0, "only right angles are supported");

                let quadrants = (action.arg / 90) as usize % 4;

                self.waypoint.turn(turn_dir, quadrants);
            }

            ActionKind::Forward => {
                for mv in self.waypoint.as_moves(action.arg).iter() {
                    self.process(mv);
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Waypoint {
    coordinates: Coordinates,
}

impl Waypoint {
    fn new() -> Self {
        Self {
            coordinates: Coordinates { x: 10, y: -1 },
        }
    }

    /// as_moves returns Actions with ActionKind::Move representing the moves the ship should take
    /// to reach the waypoint
    ///
    /// this allows reusing the Forward logic of part 1 to move the ship towards the waypoint
    fn as_moves(&self, steps: i64) -> [Action; 2] {
        let west_east = if self.coordinates.x < 0 {
            Direction::West
        } else {
            Direction::East
        };
        let north_south = if self.coordinates.y < 0 {
            Direction::North
        } else {
            Direction::South
        };

        [
            Action {
                kind: ActionKind::Move(west_east),
                arg: self.coordinates.x.abs() * steps,
            },
            Action {
                kind: ActionKind::Move(north_south),
                arg: self.coordinates.y.abs() * steps,
            },
        ]
    }

    fn turn(&mut self, turn_dir: TurnDirection, quadrants: usize) {
        let coords = &mut self.coordinates;

        for _ in 0..quadrants {
            let mut x = coords.x;
            let mut y = coords.y;

            match turn_dir {
                TurnDirection::Left => x = -x,
                TurnDirection::Right => y = -y,
            }

            coords.x = y;
            coords.y = x;
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
