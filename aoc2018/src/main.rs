use anyhow::Result;

use aoc::DayFunc;

use aoc2018::day01;
use aoc2018::day02;
use aoc2018::day03;
use aoc2018::day04;
use aoc2018::day05;

fn main() -> Result<()> {
    let days: &[DayFunc] = &[day01::run, day02::run, day03::run, day04::run, day05::run];

    aoc::run(days)
}
