use std::env;

use aoc::Result;

use aoc2015::day01;

fn main() -> Result<()> {
    let days: &[fn() -> Result<()>] = &[day01::run];

    let mut args = env::args();
    args.next();

    match args.next() {
        Some(arg) => {
            let day: usize = arg.parse().expect("Please provide a day number");
            days[day - 1]().expect("error running day specified");
        }
        None => {
            for (i, day) in days.iter().enumerate() {
                let i = i + 1;
                println!("day{}: ", i);
                day().unwrap_or_else(|e| panic!("error running day {}: {}", i, e));
                println!();
            }
        }
    }

    Ok(())
}
