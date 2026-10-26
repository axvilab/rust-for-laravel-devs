//! Exercise 5 (broken version): an overflow known at compile time
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error: this arithmetic operation will overflow
//!   (the `arithmetic_overflow` lint, deny by default)
//! Fix: `checked_add` / `saturating_add` — see examples/exercise-5.rs
// expected-error: this arithmetic operation will overflow

fn main() {
    let stock: u8 = 250;
    println!("{}", stock + 10);
}
