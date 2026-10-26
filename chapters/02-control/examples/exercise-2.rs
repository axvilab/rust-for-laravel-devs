//! Exercise 2: exhaustiveness without `_` — list the ranges explicitly
//!
//! Without `_` the compiler requires covering all of u16 (see broken/exercise-2.rs).
//! You can do it without `_`: describe the "tails" 0..=99 and 600..=u16::MAX explicitly.

fn http_class(status: u16) -> &'static str {
    match status {
        0..=99 => "invalid",
        100..=199 => "1xx",
        200..=299 => "2xx",
        300..=399 => "3xx",
        400..=499 => "4xx",
        500..=599 => "5xx",
        600..=u16::MAX => "invalid",
    }
}

fn main() {
    println!("{} {} {}", http_class(42), http_class(200), http_class(600));
}
