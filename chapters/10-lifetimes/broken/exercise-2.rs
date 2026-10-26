//! Exercise 2: `pick` without lifetimes
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0106]: missing lifetime specifier
//!   help: consider introducing a named lifetime parameter
//!   fn pick<'a>(cond: bool, a: &'a str, b: &'a str) -> &'a str
//! The compiler writes what to add itself, and marks where with plus signs.
// expected-error: E0106

fn pick(cond: bool, a: &str, b: &str) -> &str {
    if cond { a } else { b }
}

fn main() {
    println!("{}", pick(true, "yes", "no"));
}
