//! Two references in, a reference out — the compiler asks for a lifetime
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0106]: missing lifetime specifier
//!   help: consider introducing a named lifetime parameter:
//!   fn longer<'a>(a: &'a str, b: &'a str) -> &'a str
//! Fix: examples/01-longer.rs
// expected-error: E0106

fn longer(a: &str, b: &str) -> &str {
    if a.len() > b.len() { a } else { b }
}

fn main() {
    println!("{}", longer("Dnipro", "Kyiv"));
}
