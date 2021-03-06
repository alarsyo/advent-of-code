use anyhow::Result;

use aoc::DayFunc;

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
use aoc2020::day11;
use aoc2020::day12;
use aoc2020::day13;
use aoc2020::day14;
use aoc2020::day15;
use aoc2020::day16;
use aoc2020::day17;
use aoc2020::day18;
use aoc2020::day19;
use aoc2020::day20;
use aoc2020::day21;
use aoc2020::day22;
use aoc2020::day23;
use aoc2020::day24;
use aoc2020::day25;

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
        day11::run,
        day12::run,
        day13::run,
        day14::run,
        day15::run,
        day16::run,
        day17::run,
        day18::run,
        day19::run,
        day20::run,
        day21::run,
        day22::run,
        day23::run,
        day24::run,
        day25::run,
    ];

    aoc::run(days)
}
