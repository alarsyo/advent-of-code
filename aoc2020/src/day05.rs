use std::fmt::Write;
use std::str::FromStr;

use aoc::err;

const INPUT: &str = include_str!("../input/day05.txt");

pub fn run() -> aoc::Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;

    Ok(res)
}

fn part1(input: &str) -> aoc::Result<usize> {
    let seats = input
        .lines()
        .map(|line| line.parse())
        .collect::<aoc::Result<Vec<Seat>>>()
        .map_err(|e| err!("{}", e))?;

    seats
        .iter()
        .map(|seat| seat.id())
        .max()
        .ok_or_else(|| err!("0 seats processed"))
}

#[derive(Debug, PartialEq, Eq)]
struct Seat {
    row: usize,
    column: usize,
}

impl Seat {
    fn id(&self) -> usize {
        self.row * 8 + self.column
    }
}

impl FromStr for Seat {
    type Err = aoc::Error;

    fn from_str(s: &str) -> aoc::Result<Self> {
        if s.len() != 10 {
            return Err(err!("Seat encoding must be 10 chars long: {}", s));
        }

        let (mut row_begin, mut row_end) = (0, 128);
        let (mut col_begin, mut col_end) = (0, 8);
        for c in s.chars() {
            let row_range = (row_end - row_begin) / 2;
            let col_range = (col_end - col_begin) / 2;
            match c {
                'B' => {
                    row_begin += row_range;
                }
                'F' => {
                    row_end -= row_range;
                }
                'L' => {
                    col_end -= col_range;
                }
                'R' => {
                    col_begin += col_range;
                }
                _ => return Err(err!("Wrong char while decoding seat: `{}`", c)),
            }
        }

        Ok(Self {
            row: row_begin,
            column: col_begin,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_provided() {
        let seat: Seat = "BFFFBBFRRR".parse().unwrap();
        assert_eq!(seat, Seat { row: 70, column: 7 });
        assert_eq!(seat.id(), 567);

        let seat: Seat = "FFFBBBFRRR".parse().unwrap();
        assert_eq!(seat, Seat { row: 14, column: 7 });
        assert_eq!(seat.id(), 119);

        let seat: Seat = "BBFFBBFRLL".parse().unwrap();
        assert_eq!(
            seat,
            Seat {
                row: 102,
                column: 4
            }
        );
        assert_eq!(seat.id(), 820);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 850);
    }
}
