//! `Rc` не можна передати в інший потік
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0277]: `Rc<i32>` cannot be sent between threads safely
//! Виправлення: `Arc` замість `Rc` — див. examples/exercise-4.rs
// expected-error: cannot be sent between threads safely

use std::rc::Rc;
use std::thread;

fn main() {
    let n = Rc::new(5);
    let h = thread::spawn(move || println!("{n}"));
    h.join().unwrap();
}
