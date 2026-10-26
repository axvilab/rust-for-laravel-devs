//! Exercise 3: a reference to an element, then `push`, then reading the reference
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0502]: cannot borrow `v` as mutable because it is
//!   also borrowed as immutable
//! Why: `push` may reallocate the vector's memory (no room left — the data
//! moves to a new, bigger block), and `first` would point to freed memory.
//! Fix: examples/exercise-4.rs
// expected-error: E0502

fn main() {
    let mut v = vec![1, 2, 3];
    let first = &v[0];
    v.push(4);
    println!("{first}");
}
