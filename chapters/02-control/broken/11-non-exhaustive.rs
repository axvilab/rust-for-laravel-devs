//! `match` має бути вичерпним: забута гілка — помилка компіляції
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0004]: non-exhaustive patterns: `false` not covered
//! Виправлення: додати гілку `false => ...` — див. examples/11-exhaustive.rs
// expected-error: E0004

fn main() {
    let x: bool = true;
    match x {
        true => println!("так"),
    }
}
