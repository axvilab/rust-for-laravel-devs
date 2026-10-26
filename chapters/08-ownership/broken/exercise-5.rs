//! Exercise 5: `Copy` for a struct with a `String` field
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0204]: the trait `Copy` cannot be implemented for this type
//!   (this field does not implement `Copy`)
//! Why: a String keeps its data on the heap; a bitwise copy would give two owners
//! of the same piece of memory. Fix: only `Clone` — see examples/exercise-5.rs
// expected-error: E0204

#[derive(Clone, Copy)]
struct User {
    id: u64,
    name: String,
}

fn main() {}
