//! Exercise 6: `PartialEq` works, `Eq` doesn't (the `price: f64` field)
//!
//! `#[derive(PartialEq, Eq)]` gives error[E0277]: the trait bound `f64: Eq` is not satisfied
//! (see broken/exercise-6.rs): f64 has NaN, which isn't equal to itself, so
//! there's no "every value equals itself" guarantee (which is what Eq is).

#[derive(Debug, Clone, PartialEq)]
struct Product {
    id: u64,
    name: String,
    price: f64,
    stock: u32,
}

fn main() {
    let a = Product {
        id: 1,
        name: "Laptop".into(),
        price: 45_000.0,
        stock: 3,
    };
    let b = a.clone();
    println!("{}", a == b); // true
}
