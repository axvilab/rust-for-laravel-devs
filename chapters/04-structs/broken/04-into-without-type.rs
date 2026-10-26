//! `.into()` без відомого цільового типу
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0283]: type annotations needed
//! Виправлення: `let s: String = "текст".into();` — див. examples/03-string-conversions.rs
// expected-error: type annotations needed

fn main() {
    let s = "текст".into();
    println!("{s}");
}
