//! Two `&mut` at once: you can't mutate in the arguments of the same call
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0499]: cannot borrow `v` as mutable more than once at a time
//! Fix: `let last = v.pop().unwrap(); v.push(last);` — see examples/08-two-phase-borrows.rs
// expected-error: E0499

fn main() {
    let mut v: Vec<usize> = vec![1, 2, 3];
    v.push(v.pop().unwrap());
    println!("{v:?}");
}
