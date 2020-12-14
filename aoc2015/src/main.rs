use anyhow::Result;

use aoc::DayFunc;

use aoc2015::day01;
use aoc2015::day02;
use aoc2015::day03;
use aoc2015::day04;
use aoc2015::day05;
use aoc2015::day06;

fn main() -> Result<()> {
    let days: &[DayFunc] = &[
        day01::run,
        day02::run,
        day03::run,
        day04::run,
        day05::run,
        day06::run,
    ];

    aoc::run(days)
}
