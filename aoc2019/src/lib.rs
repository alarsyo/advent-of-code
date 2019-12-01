pub mod day01;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[macro_export]
macro_rules! err {
    ($($string:expr),+) => (Box::<dyn Error>::from(format!($($string),+)))
}
