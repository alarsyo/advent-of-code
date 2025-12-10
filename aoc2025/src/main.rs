use anyhow::Result;

use aoc::DayFunc;

use aoc2025::day01;
use aoc2025::day02;
use aoc2025::day03;
use aoc2025::day04;
use aoc2025::day05;
use aoc2025::day06;
use aoc2025::day07;
use aoc2025::day08;

fn main() -> Result<()> {
    let days: &[DayFunc] = &[
        day01::run,
        day02::run,
        day03::run,
        day04::run,
        day05::run,
        day06::run,
        day07::run,
        day08::run,
    ];

    aoc::run(days)
}
