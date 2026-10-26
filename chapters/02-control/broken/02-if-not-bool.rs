//! Умова `if` має бути саме `bool`: «істинності» чисел немає
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0308]: mismatched types — expected `bool`, found integer
//! Виправлення: `if count > 0 { }` — див. examples/02-if-bool.rs
// expected-error: E0308

fn main() {
    let count = 3;
    if count {} // ПОМИЛКА: expected bool, found i32
}
