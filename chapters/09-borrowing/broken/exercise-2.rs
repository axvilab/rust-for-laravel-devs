//! Exercise 2 (broken version): `&mut` to an immutable variable
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0596]: cannot borrow `s` as mutable, as it is not declared as mutable
//! Fix: `let mut s` — see examples/exercise-2.rs
// expected-error: E0596

fn append_exclamation(s: &mut String) {
    s.push('!');
}

fn main() {
    let s = String::from("Hello");
    append_exclamation(&mut s);
    println!("{s}");
}
