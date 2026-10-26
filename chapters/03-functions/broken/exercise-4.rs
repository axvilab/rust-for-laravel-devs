//! Вправа 4 (зламана версія): FnMut-замикання без `mut`
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0596]: cannot borrow `apply` as mutable, as it is not declared as mutable
//! Виправлення: `let mut apply = ...` — див. examples/exercise-4.rs
// expected-error: E0596

fn main() {
    let mut calls = 0;
    let apply = |price: f64| {
        calls += 1;
        price * 0.9
    };
    println!("{}", apply(100.0));
    println!("викликів: {calls}");
}
