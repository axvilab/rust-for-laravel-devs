//! Exercise 3 (broken version): assigning to an immutable variable
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0384]: cannot assign twice to immutable variable `price`
//! Fix: `let mut price` — see examples/exercise-3.rs
// expected-error: E0384

fn main() {
    let price = 100.0;
    price *= 0.9;
    println!("{price}");
}
