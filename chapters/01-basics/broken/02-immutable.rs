//! Змінні незмінні за замовчуванням: повторне присвоєння не компілюється
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0384]: cannot assign twice to immutable variable `count`
//! Виправлення: `let mut count = 5;` — див. examples/02-mut.rs
// expected-error: E0384

fn main() {
    let count = 5;
    count = count + 1; // ПОМИЛКА: cannot assign twice to immutable variable
    println!("{count}");
}
