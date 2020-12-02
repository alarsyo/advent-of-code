use aoc::DayFunc;
use aoc::Result;

use aoc2020::day01;
use aoc2020::day02;

fn main() -> Result<()> {
    let days: &[DayFunc] = &[day01::run, day02::run];

    aoc::run(days)
}
