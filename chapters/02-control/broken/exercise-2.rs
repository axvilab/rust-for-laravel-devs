//! Exercise 2: without the `_` arm a match on u16 is not exhaustive
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0004]: non-exhaustive patterns:
//!   `0_u16..=99_u16` and `600_u16..=u16::MAX` not covered
//! The compiler lists exactly which values are not covered.
//! Fix: bring back `_ => "invalid"` — see examples/exercise-1.rs
// expected-error: E0004

fn http_class(status: u16) -> &'static str {
    match status {
        100..=199 => "1xx",
        200..=299 => "2xx",
        300..=399 => "3xx",
        400..=499 => "4xx",
        500..=599 => "5xx",
    }
}

fn main() {
    println!("{}", http_class(200));
}
