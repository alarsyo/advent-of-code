use std::env;

use anyhow::{Context, Result};

pub type DayFunc = fn() -> Result<String>;

pub fn run(days: &[DayFunc]) -> Result<()> {
    let mut args = env::args();
    args.next();

    match args.next() {
        Some(arg) => {
            let day: usize = arg.parse().context("couldn't parse day number")?;
            let res = days[day - 1]().context("error running day specified")?;
            println!("{}", res);
        }
        None => {
            for (i, day) in days.iter().enumerate() {
                let i = i + 1;
                println!("day{}: ", i);
                let res = day().with_context(|| format!("error running day {}", i))?;
                println!("{}", res);
            }
        }
    }

    Ok(())
}
