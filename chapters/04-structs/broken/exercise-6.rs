//! Exercise 6 (broken version): `Eq` on a struct with an `f64`
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0277]: the trait bound `f64: Eq` is not satisfied
//! Fix: remove `Eq` — see examples/exercise-6.rs
// expected-error: E0277

#[derive(Debug, PartialEq, Eq)]
struct Product {
    id: u64,
    price: f64,
}

fn main() {
    let p = Product { id: 1, price: 1.0 };
    println!("{}", p == p);
}
