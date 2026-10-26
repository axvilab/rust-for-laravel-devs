//! Exercise 6 (broken version): `v.push(v.pop().unwrap())`
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0499]: cannot borrow `v` as mutable more than once at a time
//! Fix: a temporary variable — see examples/exercise-6.rs
// expected-error: E0499

fn main() {
    let mut v: Vec<usize> = vec![1, 2, 3];
    v.push(v.pop().unwrap());
    println!("{v:?}");
}
