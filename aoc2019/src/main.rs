use aoc::DayFunc;
use aoc::Result;

use aoc2019::day01;
use aoc2019::day02;
use aoc2019::day03;
use aoc2019::day04;
use aoc2019::day05;
use aoc2019::day06;
use aoc2019::day07;
use aoc2019::day08;
use aoc2019::day09;
use aoc2019::day10;

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
