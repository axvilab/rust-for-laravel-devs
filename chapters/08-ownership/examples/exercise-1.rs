//! Exercise 1: `let t = s;` — and `s` is no longer available
//!
//! `println!("{s}")` after `let t = s;` gives error[E0382]: borrow of moved value: `s`
//! with markers: `value moved here` — the `let t = s;` line, and
//! `value borrowed here after move` — the place of use (see broken/exercise-1.rs).

fn main() {
    let s = String::from("test");
    let t = s;
    println!("{t}"); // t is the owner now
}
