//! Exercise 3: without `mut`, reassignment doesn't compile
//!
//! If you remove `mut`, the compiler says:
//!   error[E0384]: cannot assign twice to immutable variable `price`
//!   help: consider making this binding mutable: `mut price`
//! So it suggests the fix itself — and that fix is what you see below.
//! Broken version: broken/exercise-3.rs

fn main() {
    let mut price = 100.0;
    price *= 0.9;
    println!("{price}");
}
