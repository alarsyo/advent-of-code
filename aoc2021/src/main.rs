use anyhow::Result;

use aoc::DayFunc;

use aoc2021::day01;

fn main() -> Result<()> {
    let days: &[DayFunc] = &[day01::run];

    aoc::run(days)
}
