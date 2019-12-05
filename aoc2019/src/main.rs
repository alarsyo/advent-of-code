use aoc::Result;

use aoc2019::day01;
use aoc2019::day02;
use aoc2019::day03;
use aoc2019::day04;
use aoc2019::day05;

fn main() -> Result<()> {
    let days: &[fn() -> Result<()>] = &[day01::run, day02::run, day03::run, day04::run, day05::run];

    aoc::run(days)
}
