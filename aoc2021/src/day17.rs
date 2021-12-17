use std::cmp::Ordering;
use std::fmt::Write;
use std::ops::RangeInclusive;

use anyhow::{Context, Result};

const INPUT: &str = include_str!("../input/day17.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;
    writeln!(res, "part 2: {}", part2(INPUT)?)?;

    Ok(res)
}

fn part1(input: &str) -> Result<isize> {
    let area: TargetArea = input
        .parse()
        .context("couldn't parse input to target area")?;

    let min_x_vel = if area.min_x() > 0 {
        (0..).find(|x| ((x * (x + 1)) / 2) >= area.min_x()).unwrap()
    } else if area.max_x() < 0 {
        -(0..)
            .find(|x| ((x * (x + 1)) / 2) >= area.max_x().abs())
            .unwrap()
    } else {
        0
    };

    let max_x_vel = if area.min_x() > 0 {
        area.max_x()
    } else if area.max_x() < 0 {
        area.min_x()
    } else {
        0
    };

    // Rust ranges can only be increasing, so swap values around if negative
    let (min_x_vel, max_x_vel) = (min_x_vel.min(max_x_vel), min_x_vel.max(max_x_vel));

    // we could launch the prob downward, but in that case max Y reached would always be 0
    let min_y_vel = 1;
    let max_y_vel = area.min_y().abs();

    (min_x_vel..=max_x_vel)
        .flat_map(|x_vel| (min_y_vel..=max_y_vel).map(move |y_vel| (x_vel, y_vel)))
        .filter_map(|(x_vel, y_vel)| throw(x_vel, y_vel, &area))
        .max()
        .context("couldn't find any trajectory")
}

fn part2(input: &str) -> Result<usize> {
    let area: TargetArea = input
        .parse()
        .context("couldn't parse input to target area")?;

    let min_x_vel = if area.min_x() > 0 {
        (0..).find(|x| ((x * (x + 1)) / 2) >= area.min_x()).unwrap()
    } else if area.max_x() < 0 {
        -(0..)
            .find(|x| ((x * (x + 1)) / 2) >= area.max_x().abs())
            .unwrap()
    } else {
        0
    };

    let max_x_vel = if area.min_x() > 0 {
        area.max_x()
    } else if area.max_x() < 0 {
        area.min_x()
    } else {
        0
    };

    // Rust ranges can only be increasing, so swap values around if negative
    let (min_x_vel, max_x_vel) = (min_x_vel.min(max_x_vel), min_x_vel.max(max_x_vel));

    // let's assume that the area is always lower than (0, 0)
    let min_y_vel = area.min_y();
    let max_y_vel = area.min_y().abs();

    Ok((min_x_vel..=max_x_vel)
        .flat_map(|x_vel| (min_y_vel..=max_y_vel).map(move |y_vel| (x_vel, y_vel)))
        .filter_map(|(x_vel, y_vel)| throw(x_vel, y_vel, &area))
        .count())
}

fn throw(mut xvel: isize, mut yvel: isize, area: &TargetArea) -> Option<isize> {
    let (mut pos_x, mut pos_y) = (0, 0);
    let mut highest_y = 0;

    loop {
        if area.contains(pos_x, pos_y) {
            return Some(highest_y);
        }

        // Three cases where we can stop here:
        // - probe is lower than area, and gravity pulls it even lower
        // - probe is on the left, and x velocity goes left
        // - probe is on the right, and x velocity goes right
        if (pos_y < area.min_y() && yvel <= 0)
            || (xvel <= 0 && pos_x < area.min_x())
            || (xvel >= 0 && pos_x > area.max_x())
        {
            // the probe will never reach the area, we can stop the simulation here
            return None;
        }

        // - The probe's x position increases by its x velocity.
        pos_x += xvel;
        // - The probe's y position increases by its y velocity.
        pos_y += yvel;
        // - Due to drag, the probe's x velocity changes by 1 toward the value 0; that is, it
        // decreases by 1 if it is greater than 0, increases by 1 if it is less than 0, or does not
        // change if it is already 0.
        match xvel.cmp(&0) {
            Ordering::Less => xvel += 1,
            Ordering::Greater => xvel -= 1,
            _ => {}
        }
        // - Due to gravity, the probe's y velocity decreases by 1.
        yvel -= 1;

        // update highest seen y
        highest_y = highest_y.max(pos_y);
    }
}

struct TargetArea {
    x_range: RangeInclusive<isize>,
    y_range: RangeInclusive<isize>,
}

impl TargetArea {
    fn contains(&self, x: isize, y: isize) -> bool {
        self.x_range.contains(&x) && self.y_range.contains(&y)
    }

    fn min_y(&self) -> isize {
        *self.y_range.start()
    }

    fn min_x(&self) -> isize {
        *self.x_range.start()
    }

    fn max_x(&self) -> isize {
        *self.x_range.end()
    }
}

impl std::str::FromStr for TargetArea {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let s = s
            .trim()
            .strip_prefix("target area: ")
            .context("missing target area while parsing")?;

        let (x, y) = s.split_once(", ").context("couldn't split on comma")?;

        let x = x.strip_prefix("x=").context("couldn't find `x=`")?;
        let (min_x, max_x) = x.split_once("..").context("couldn't split on `..`")?;
        let (min_x, max_x) = (min_x.parse()?, max_x.parse()?);

        let y = y.strip_prefix("y=").context("couldn't find `y=`")?;
        let (min_y, max_y) = y.split_once("..").context("couldn't split on `..`")?;
        let (min_y, max_y) = (min_y.parse()?, max_y.parse()?);

        Ok(Self {
            x_range: min_x..=max_x,
            y_range: min_y..=max_y,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED: &str = include_str!("../input/day17_provided.txt");

    #[test]
    fn part1_provided() {
        assert_eq!(part1(PROVIDED).unwrap(), 45);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 4186);
    }

    #[test]
    fn part2_provided() {
        assert_eq!(part2(PROVIDED).unwrap(), 112);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT).unwrap(), 2709);
    }
}
