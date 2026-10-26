//! Вправа 3 (зламана версія): присвоєння незмінній змінній
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0384]: cannot assign twice to immutable variable `price`
//! Виправлення: `let mut price` — див. examples/exercise-3.rs
// expected-error: E0384

fn main() {
    let price = 100.0;
    price *= 0.9;
    println!("{price}");
}
