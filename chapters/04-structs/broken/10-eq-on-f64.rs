//! `#[derive(Eq)]` на структурі з `f64`
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0277]: the trait bound `f64: Eq` is not satisfied
//! Виправлення: лише `PartialEq`, або гроші в копійках як `i64` — див. examples/10-partial-eq.rs
// expected-error: E0277

#[derive(Debug, PartialEq, Eq)]
struct Price {
    amount: f64,
}

fn main() {
    println!("{:?}", Price { amount: 1.0 });
}
