//! The same, but `first` is used AFTER `push`
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0502]: cannot borrow `scores` as mutable because it is
//!   also borrowed as immutable
//! Fix: use the reference earlier — see examples/07-nll.rs
// expected-error: E0502

fn main() {
    let mut scores = vec![10, 20];
    let first = &scores[0];
    scores.push(30);
    println!("{first}");
}
