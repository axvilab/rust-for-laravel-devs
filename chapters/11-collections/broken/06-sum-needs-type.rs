//! `.sum()` без підказки типу
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0283]: type annotations needed
//! Виправлення: `let total: f64 = ...` або `.sum::<f64>()` — див. examples/06-iterator-chain.rs
// expected-error: E0283

fn main() {
    let prices = [100.0, 250.0];
    let total = prices.iter().sum();
    println!("{total}");
}
