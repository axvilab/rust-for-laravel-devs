//! You can't change a value through `&`
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0594]: cannot assign to `*s`, which is behind a `&` reference
//! Fix: `&mut` in the reference and `let mut` on the owner — see examples/04-mut-binding-vs-mut-ref.rs
// expected-error: E0594

fn main() {
    let b = 2;
    let s = &b;
    *s += 1;
    println!("{s}");
}
