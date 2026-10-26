//! `let b = a` for a String is a move: `a` can't be used any more
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0382]: borrow of moved value: `a`
//! Fix: use `b`, or `let b = a.clone();` — see examples/02-move.rs
// expected-error: E0382

fn main() {
    let a = String::from("hello");
    let b = a; // ownership MOVED to b
    println!("{a}"); // ERROR: borrow of moved value: `a`
    println!("{b}");
}
