//! You can't return a reference to a local variable
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0106]: missing lifetime specifier
//!   (with two `help:` hints — add `'static` (a trap!) or return a `String`)
//! Fix: return ownership — see examples/09-return-owned.rs
// expected-error: E0106

fn longest_name() -> &str {
    let name = String::from("Vitalii");
    &name // ERROR: returning a reference to something that's about to be destroyed
}

fn main() {
    println!("{}", longest_name());
}
