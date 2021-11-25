use anyhow::Result;

use aoc::DayFunc;

use aoc2021::day00;

fn main() -> Result<()> {
    let days: &[DayFunc] = &[day00::run];

    aoc::run(days)
}
