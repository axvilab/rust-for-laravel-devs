//! An extra `;` after the last expression: the function returns `()` instead of i32
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0308]: mismatched types — expected `i32`, found `()`
//!   with the hint `help: remove this semicolon to return this value`
//! Fix: remove the `;` after `n * n` — see examples/13-return.rs
// expected-error: E0308

fn square(n: i32) -> i32 {
    n * n;
}

fn main() {
    println!("{}", square(4));
}
