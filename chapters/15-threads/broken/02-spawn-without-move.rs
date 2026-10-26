//! Потік позичає локальну змінну — вона може не дожити
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0373]: closure may outlive the current function, but it
//!   borrows `name`, which is owned by the current function
//!   help: to force the closure to take ownership of `name` ... use the `move` keyword
//! Виправлення: `thread::spawn(move || ...)` — див. examples/02-spawn-move.rs
// expected-error: E0373

use std::thread;

fn main() {
    let name = String::from("Віталій");
    let h = thread::spawn(|| {
        println!("{name}");
    });
    h.join().unwrap();
}
