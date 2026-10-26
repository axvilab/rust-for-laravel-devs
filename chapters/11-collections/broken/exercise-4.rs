//! Exercise 4 (broken version): using `v` after `let v2 = v;`
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0382]: borrow of moved value: `v`
//! Fix: `let v2 = v.clone();` — see examples/exercise-4.rs
// expected-error: E0382

fn main() {
    let v = vec![1, 2, 3];
    let v2 = v;
    println!("{v:?} {v2:?}");
}
