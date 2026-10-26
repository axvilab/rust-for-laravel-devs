//! `Arc::clone(&x)` для значення, яке не в `Arc`, не компілюється — і це плюс
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0308]: mismatched types (expected `&Arc<_>`, found `&Config`)
//! Виправлення: `Arc::new(Config { .. })` — див. examples/09-arc-clone.rs
// expected-error: E0308

use std::sync::Arc;

#[derive(Clone)]
struct Config {
    port: u16,
}

fn main() {
    let config = Config { port: 3000 };
    let copy = Arc::clone(&config);
    println!("{}", copy.port);
}
