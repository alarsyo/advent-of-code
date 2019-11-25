pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[macro_export]
macro_rules! err {
    ($($string:expr),+) => (Box::<dyn Error>::from(format!($($string),+)))
}
