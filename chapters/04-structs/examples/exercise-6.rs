//! Вправа 6: `PartialEq` працює, `Eq` — ні (поле `price: f64`)
//!
//! `#[derive(PartialEq, Eq)]` дає error[E0277]: the trait bound `f64: Eq` is not satisfied
//! (див. broken/exercise-6.rs): f64 має NaN, який не дорівнює сам собі, тож
//! гарантії «кожне значення дорівнює собі» (а це і є Eq) немає.

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
        name: "Ноутбук".into(),
        price: 45_000.0,
        stock: 3,
    };
    let b = a.clone();
    println!("{}", a == b); // true
}
