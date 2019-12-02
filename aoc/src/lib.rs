use std::env;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[macro_export]
macro_rules! err {
    ($($string:expr),+) => (Box::<dyn std::error::Error>::from(format!($($string),+)))
}

type DayFunc = fn() -> Result<()>;

pub fn run(days: &[DayFunc]) -> Result<()> {
    let mut args = env::args();
    args.next();

    match args.next() {
        Some(arg) => {
            let day: usize = arg.parse().expect("Please provide a day number");
            days[day - 1]().unwrap_or_else(|e| eprintln!("error running day specified: {}", e));
        }
        None => {
            for (i, day) in days.iter().enumerate() {
                let i = i + 1;
                println!("day{}: ", i);
                day().unwrap_or_else(|e| eprintln!("error running day {}: {}", i, e));
                println!();
            }
        }
    }

    Ok(())
}
