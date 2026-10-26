//! `match` must be exhaustive: a forgotten arm is a compile error
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0004]: non-exhaustive patterns: `false` not covered
//! Fix: add a `false => ...` arm — see examples/11-exhaustive.rs
// expected-error: E0004

fn main() {
    let x: bool = true;
    match x {
        true => println!("yes"),
    }
}
