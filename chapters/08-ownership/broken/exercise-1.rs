//! Exercise 1 (broken version): use after move
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0382]: borrow of moved value: `s`
//! Fix: see examples/exercise-1.rs
// expected-error: E0382

fn main() {
    let s = String::from("test");
    let t = s;
    println!("{s} {t}");
}
