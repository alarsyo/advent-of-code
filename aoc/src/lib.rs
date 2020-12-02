use std::env;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[macro_export]
macro_rules! err {
    ($($string:expr),+) => (Box::<dyn std::error::Error>::from(format!($($string),+)))
}

pub type DayFunc = fn() -> Result<String>;

pub fn run(days: &[DayFunc]) -> Result<()> {
    let mut args = env::args();
    args.next();

    match args.next() {
        Some(arg) => {
            let day: usize = arg.parse().expect("Please provide a day number");
            let res = days[day - 1]().map_err(|e| err!("error running day specified: {}", e))?;
            println!("{}", res);
        }
        None => {
            for (i, day) in days.iter().enumerate() {
                let i = i + 1;
                println!("day{}: ", i);
                let res = day().map_err(|e| err!("error running day {}: {}", i, e))?;
                println!("{}", res);
            }
        }
    }

    Ok(())
}
