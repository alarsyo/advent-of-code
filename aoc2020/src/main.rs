use aoc::DayFunc;
use aoc::Result;

use aoc2020::day01;
use aoc2020::day02;
use aoc2020::day03;
use aoc2020::day04;
use aoc2020::day05;
use aoc2020::day06;
use aoc2020::day07;
use aoc2020::day08;
use aoc2020::day09;
use aoc2020::day10;

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
        day09::run,
        day10::run,
    ];

    aoc::run(days)
}
