//! Exercise 5: without `Debug` in `derive`, printing with `{:?}` doesn't compile
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0277]: `Product` doesn't implement `Debug`
//!   note: add `#[derive(Debug)]` to `Product` or manually `impl Debug for Product`
//! The `note:` line is the ready-made hint. Fix: examples/exercise-4.rs
// expected-error: E0277

struct Product {
    id: u64,
    name: String,
}

fn main() {
    let p = Product {
        id: 1,
        name: "Laptop".into(),
    };
    println!("{p:#?}");
}
