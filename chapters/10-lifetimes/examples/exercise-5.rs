//! Exercise 5: `make_greeting` returns a `String`
//!
//! With `-> &str` and `&format!(...)` the compiler gives E0515 "cannot return reference
//! to temporary value" (broken/exercise-5.rs): the string from format! is destroyed when
//! the function returns. No `'a` will help — you have to give away the string itself.

fn make_greeting(name: &str) -> String {
    format!("Hello, {name}")
}

fn main() {
    let g = make_greeting("Vitalii");
    println!("{g}");
}
