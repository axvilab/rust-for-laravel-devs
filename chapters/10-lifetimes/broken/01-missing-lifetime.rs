//! Два посилання на вході, посилання на виході — компілятор просить лайфтайм
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0106]: missing lifetime specifier
//!   help: consider introducing a named lifetime parameter:
//!   fn longer<'a>(a: &'a str, b: &'a str) -> &'a str
//! Виправлення: examples/01-longer.rs
// expected-error: E0106

fn longer(a: &str, b: &str) -> &str {
    if a.len() > b.len() { a } else { b }
}

fn main() {
    println!("{}", longer("Дніпро", "Київ"));
}
