//! Exercise 3 (broken version): `Option<Reply>` inside `Reply`
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0072]: recursive type `Reply` has infinite size
//!   help: insert some indirection … `parent: Option<Box<Reply>>`
//! Fix: examples/exercise-3.rs
// expected-error: E0072

struct Reply {
    text: String,
    parent: Option<Reply>,
}

fn main() {}
