//! Exercise 5: returning `&format!(...)` from a function
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0515]: cannot return reference to temporary value
//! Fix: return a `String` — see examples/exercise-5.rs
// expected-error: E0515

fn make_greeting(name: &str) -> &str {
    &format!("Hello, {name}")
}

fn main() {
    println!("{}", make_greeting("Vitalii"));
}
