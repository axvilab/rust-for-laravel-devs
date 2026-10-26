//! Лайфтайм не врятує посилання на локальну змінну
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0515]: cannot return reference to local variable `s`
//! Виправлення: повертати `String` — див. examples/03-return-owned.rs
// expected-error: E0515

fn make_greeting(name: &str) -> &str {
    let s = format!("Привіт, {name}");
    &s // ПОМИЛКА E0515
}

fn main() {
    println!("{}", make_greeting("Віталій"));
}
