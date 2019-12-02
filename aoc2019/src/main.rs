use aoc::Result;

use aoc2019::day01;
use aoc2019::day02;

fn main() -> Result<()> {
    let days: &[fn() -> Result<()>] = &[day01::run, day02::run];

    aoc::run(days)
}
