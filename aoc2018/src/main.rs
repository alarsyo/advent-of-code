use std::env;

use aoc2018::day01;
use aoc2018::day02;
use aoc2018::day03;
use aoc2018::day05;

use aoc2018::Result;

fn main() -> Result<()> {
    let days: &[fn() -> Result<()>] = &[day01::run, day02::run, day03::run, day05::run];

    let mut args = env::args();
    args.next();

    match args.next() {
        Some(arg) => {
            let day: usize = arg.parse().expect("Please provide a day number");
            days[day - 1]().expect("error running day specified");
        }
        None => {
            for (i, day) in days.iter().enumerate() {
                println!("day{}: ", i);
                day().unwrap_or_else(|e| panic!("error running day {}: {}", i, e));
                println!();
            }
        }
    }

    Ok(())
}
