use aoc::Result;

use aoc2019::day01;
use aoc2019::day02;
use aoc2019::day03;

fn main() -> Result<()> {
    let days: &[fn() -> Result<()>] = &[day01::run, day02::run, day03::run];

    aoc::run(days)
}
