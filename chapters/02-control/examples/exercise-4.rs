//! Exercise 4: `_` at the top of a `match` — the function always returns "invalid"
//!
//! The compiler warns `unreachable pattern` on every arm below `_`:
//! arms are checked top to bottom and `_` matches any value,
//! so execution never reaches 100..=199 and the rest.
//! The `allow` is only for CI; remove it to see the warning.
#![allow(unreachable_patterns)]

fn http_class(status: u16) -> &'static str {
    match status {
        _ => "invalid",
        100..=199 => "1xx",
        200..=299 => "2xx",
        300..=399 => "3xx",
        400..=499 => "4xx",
        500..=599 => "5xx",
    }
}

fn main() {
    println!("{}", http_class(200)); // invalid — the 200..=299 arm is unreachable
}
