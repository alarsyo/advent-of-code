use std::env;

use aoc2018::day01;
use aoc2018::day02;
use aoc2018::Result;

fn main() -> Result<()> {
    let days = [day01::run, day02::run];

    let mut args = env::args();
    args.next();

    let day = args
        .next()
        .expect("Please provide a day to launch")
        .parse::<usize>()?;

    days[day - 1]()
}
