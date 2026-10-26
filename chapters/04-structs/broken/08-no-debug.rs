//! `{:?}` для структури без `#[derive(Debug)]`
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0277]: `User` doesn't implement `Debug`
//!   note: add `#[derive(Debug)]` to `User` or manually `impl Debug for User`
//! Виправлення: `#[derive(Debug)]` — див. examples/08-derive-debug.rs
// expected-error: E0277

struct User {
    id: u64,
    name: String,
}

fn main() {
    let user = User {
        id: 1,
        name: "Віталій".into(),
    };
    println!("{:?}", user);
}
