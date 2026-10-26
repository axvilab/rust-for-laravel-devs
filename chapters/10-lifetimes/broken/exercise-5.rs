//! Вправа 5: повернути `&format!(...)` з функції
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0515]: cannot return reference to temporary value
//! Виправлення: повертати `String` — див. examples/exercise-5.rs
// expected-error: E0515

fn make_greeting(name: &str) -> &str {
    &format!("Привіт, {name}")
}

fn main() {
    println!("{}", make_greeting("Віталій"));
}
