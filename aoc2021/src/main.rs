use anyhow::Result;

use aoc::DayFunc;

use aoc2021::day01;
use aoc2021::day02;
use aoc2021::day03;
use aoc2021::day04;

fn main() -> Result<()> {
    let days: &[DayFunc] = &[day01::run, day02::run, day03::run, day04::run];

    aoc::run(days)
}
