//! Вправа 2: `append_exclamation(&mut String)` вимагає `let mut` у власника
//!
//! Без `mut`: error[E0596]: cannot borrow `s` as mutable, as it is not declared as mutable
//! (див. broken/exercise-2.rs) — і `help:` підказує дописати `mut`.

fn append_exclamation(s: &mut String) {
    s.push('!');
}

fn main() {
    let mut s = String::from("Привіт");
    append_exclamation(&mut s);
    println!("{s}"); // Привіт!
}
