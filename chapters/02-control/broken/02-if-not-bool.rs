//! An `if` condition must be a `bool`: numbers have no "truthiness"
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0308]: mismatched types — expected `bool`, found integer
//! Fix: `if count > 0 { }` — see examples/02-if-bool.rs
// expected-error: E0308

fn main() {
    let count = 3;
    if count {} // ERROR: expected bool, found i32
}
