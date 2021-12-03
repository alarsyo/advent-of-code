use anyhow::Result;

use aoc::DayFunc;

use aoc2021::day01;
use aoc2021::day02;
use aoc2021::day03;

fn main() -> Result<()> {
    let days: &[DayFunc] = &[day01::run, day02::run, day03::run];

    aoc::run(days)
}
