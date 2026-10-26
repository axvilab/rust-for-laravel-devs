//! Зайва `;` після останнього виразу: функція повертає `()` замість i32
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0308]: mismatched types — expected `i32`, found `()`
//!   з підказкою `help: remove this semicolon to return this value`
//! Виправлення: прибрати `;` після `n * n` — див. examples/13-return.rs
// expected-error: E0308

fn square(n: i32) -> i32 {
    n * n;
}

fn main() {
    println!("{}", square(4));
}
