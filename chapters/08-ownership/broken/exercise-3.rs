//! Exercise 3 (broken version): the string is used after being passed to `shout`
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0382]: borrow of moved value: `s`
//! Fix: see examples/exercise-3.rs
// expected-error: E0382

fn shout(s: String) -> String {
    s.to_uppercase()
}

fn main() {
    let s = String::from("hello");
    let loud = shout(s);
    println!("{s} → {loud}");
}
