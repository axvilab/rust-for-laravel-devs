//! Exercise 7 (broken version): `.into()` without a type annotation
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0283]: type annotations needed
//! Fix: `let p: Product = 99.9.into();` — see examples/exercise-7.rs
// expected-error: type annotations needed

#[derive(Debug)]
struct Product {
    price: f64,
}

impl From<f64> for Product {
    fn from(price: f64) -> Self {
        Product { price }
    }
}

fn main() {
    let p = 99.9.into();
    println!("{p:?}");
}
