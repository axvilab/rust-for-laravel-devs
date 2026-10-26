//! Вправа 6 (зламана версія): `Eq` на структурі з `f64`
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0277]: the trait bound `f64: Eq` is not satisfied
//! Виправлення: прибрати `Eq` — див. examples/exercise-6.rs
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
