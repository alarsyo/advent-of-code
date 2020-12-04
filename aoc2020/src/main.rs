use aoc::DayFunc;
use aoc::Result;

use aoc2020::day01;
use aoc2020::day02;
use aoc2020::day03;
use aoc2020::day04;

fn main() -> Result<()> {
    let days: &[DayFunc] = &[day01::run, day02::run, day03::run, day04::run];

    aoc::run(days)
}
