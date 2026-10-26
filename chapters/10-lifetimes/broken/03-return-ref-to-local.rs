//! A lifetime won't save a reference to a local variable
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0515]: cannot return reference to local variable `s`
//! Fix: return a `String` — see examples/03-return-owned.rs
// expected-error: E0515

fn make_greeting(name: &str) -> &str {
    let s = format!("Hello, {name}");
    &s // ERROR E0515
}

fn main() {
    println!("{}", make_greeting("Vitalii"));
}
