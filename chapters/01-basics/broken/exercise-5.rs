//! Вправа 5 (зламана версія): переповнення, відоме під час компіляції
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error: this arithmetic operation will overflow
//!   (лінт `arithmetic_overflow`, увімкнений як deny)
//! Виправлення: `checked_add` / `saturating_add` — див. examples/exercise-5.rs
// expected-error: this arithmetic operation will overflow

fn main() {
    let stock: u8 = 250;
    println!("{}", stock + 10);
}
