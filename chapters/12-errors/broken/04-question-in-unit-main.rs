//! `?` у звичайному `main`, який нічого не повертає
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0277]: the `?` operator can only be used in a function
//!   that returns `Result` or `Option`
//! Виправлення: `fn main() -> Result<(), ParseIntError>` — див. examples/04-main-result.rs
// expected-error: E0277

fn main() {
    let port: u16 = "80a".parse()?;
    println!("порт {port}");
}
