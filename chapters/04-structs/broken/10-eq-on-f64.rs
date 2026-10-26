//! `#[derive(Eq)]` on a struct with an `f64`
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0277]: the trait bound `f64: Eq` is not satisfied
//! Fix: only `PartialEq`, or money in cents as `i64` — see examples/10-partial-eq.rs
// expected-error: E0277

#[derive(Debug, PartialEq, Eq)]
struct Price {
    amount: f64,
}

fn main() {
    println!("{:?}", Price { amount: 1.0 });
}
