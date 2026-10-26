//! `.into()` without a known target type
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0283]: type annotations needed
//! Fix: `let s: String = "text".into();` — see examples/03-string-conversions.rs
// expected-error: type annotations needed

fn main() {
    let s = "text".into();
    println!("{s}");
}
