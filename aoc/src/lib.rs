pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[macro_export]
macro_rules! err {
    ($($string:expr),+) => (Box::<dyn std::error::Error>::from(format!($($string),+)))
}
