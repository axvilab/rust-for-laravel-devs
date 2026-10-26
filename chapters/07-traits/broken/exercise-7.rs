//! Вправа 7 (зламана версія): `.into()` без анотації типу
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0283]: type annotations needed
//! Виправлення: `let p: Product = 99.9.into();` — див. examples/exercise-7.rs
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
