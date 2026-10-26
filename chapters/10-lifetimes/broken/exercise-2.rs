//! Вправа 2: `pick` без лайфтаймів
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0106]: missing lifetime specifier
//!   help: consider introducing a named lifetime parameter
//!   fn pick<'a>(cond: bool, a: &'a str, b: &'a str) -> &'a str
//! Компілятор сам пише, що вписати, і плюсиками позначає, куди.
// expected-error: E0106

fn pick(cond: bool, a: &str, b: &str) -> &str {
    if cond { a } else { b }
}

fn main() {
    println!("{}", pick(true, "так", "ні"));
}
