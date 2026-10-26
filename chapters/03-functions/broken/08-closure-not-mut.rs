//! Замикання, яке змінює зовнішню змінну, саме має бути `let mut`
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0596]: cannot borrow `inc` as mutable, as it is not declared as mutable
//!   з підказкою `help: consider changing this to be mutable: mut inc`
//! Виправлення: `let mut inc = ...` — див. examples/08-fnmut-counter.rs
// expected-error: E0596

fn main() {
    let mut count = 0;
    let inc = || count += 1;
    inc();
    inc();
    println!("{count}");
}
