//! Exercise 7: `From<f64> for Product`; `.into()` needs a type annotation
//!
//! `let p = 99.9.into();` without a type gives error[E0283]: type annotations needed
//! (see broken/exercise-7.rs): `.into()` could turn an f64 into many types,
//! and the compiler doesn't guess. The fix is to name the type, one of two ways.

#[derive(Debug)]
struct Product {
    name: String,
    price: f64,
}

impl From<f64> for Product {
    fn from(price: f64) -> Self {
        Product {
            name: "Untitled".to_string(),
            price,
        }
    }
}

fn main() {
    let p: Product = 99.9.into(); // way 1: annotate the variable
    let q = Product::from(10.0); // way 2: From directly — more readable
    println!("{p:?}\n{q:?}");
}
