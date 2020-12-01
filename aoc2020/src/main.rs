use aoc::DayFunc;
use aoc::Result;

use aoc2020::day01;

fn main() -> Result<()> {
    let days: &[DayFunc] = &[day01::run];

    aoc::run(days)
}
