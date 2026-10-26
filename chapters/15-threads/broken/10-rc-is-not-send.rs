//! An `Rc` can't be sent to another thread
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0277]: `Rc<i32>` cannot be sent between threads safely
//! Fix: `Arc` instead of `Rc` — see examples/exercise-4.rs
// expected-error: cannot be sent between threads safely

use std::rc::Rc;
use std::thread;

fn main() {
    let n = Rc::new(5);
    let h = thread::spawn(move || println!("{n}"));
    h.join().unwrap();
}
