//! Variables are immutable by default: reassignment does not compile
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0384]: cannot assign twice to immutable variable `count`
//! Fix: `let mut count = 5;` — see examples/02-mut.rs
// expected-error: E0384

fn main() {
    let count = 5;
    count = count + 1; // ERROR: cannot assign twice to immutable variable
    println!("{count}");
}
