//! Exercise 2: `append_exclamation(&mut String)` needs `let mut` on the owner
//!
//! Without `mut`: error[E0596]: cannot borrow `s` as mutable, as it is not declared as mutable
//! (see broken/exercise-2.rs) — and `help:` suggests adding `mut`.

fn append_exclamation(s: &mut String) {
    s.push('!');
}

fn main() {
    let mut s = String::from("Hello");
    append_exclamation(&mut s);
    println!("{s}"); // Hello!
}
