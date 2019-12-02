use aoc::Result;

use aoc2015::day01;

fn main() -> Result<()> {
    let days: &[fn() -> Result<()>] = &[day01::run];

    aoc::run(days)
}
