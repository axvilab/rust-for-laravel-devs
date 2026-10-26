//! `==` для enum без `PartialEq`
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0369]: binary operation `==` cannot be applied to type `Status`
//! Виправлення: `#[derive(PartialEq)]` або `matches!` — див. examples/05-enum-eq.rs
// expected-error: E0369

enum Status {
    Pending,
    Paid { amount: i64 },
}

fn main() {
    let s = Status::Paid { amount: 1500 };
    if s == Status::Pending {
        println!("чекаємо");
    }
}
