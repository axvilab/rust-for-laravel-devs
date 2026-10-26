//! `async fn main` без `#[tokio::main]`
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0752]: `main` function is not allowed to be `async`
//! Виправлення: `#[tokio::main]` над `async fn main` — див. examples/02-tokio-main.rs
// expected-error: E0752

async fn main() {
    println!("привіт");
}
