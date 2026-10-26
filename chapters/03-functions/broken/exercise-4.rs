//! Exercise 4 (broken version): an FnMut closure without `mut`
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0596]: cannot borrow `apply` as mutable, as it is not declared as mutable
//! Fix: `let mut apply = ...` — see examples/exercise-4.rs
// expected-error: E0596

fn main() {
    let mut calls = 0;
    let apply = |price: f64| {
        calls += 1;
        price * 0.9
    };
    println!("{}", apply(100.0));
    println!("calls: {calls}");
}
