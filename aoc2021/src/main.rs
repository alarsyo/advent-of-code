use anyhow::Result;

use aoc::DayFunc;

use aoc2021::day01;
use aoc2021::day02;

fn main() -> Result<()> {
    let days: &[DayFunc] = &[day01::run, day02::run];

    aoc::run(days)
}
