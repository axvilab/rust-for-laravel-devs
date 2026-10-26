//! A function's value is its last expression; `return` works but isn't idiomatic
// clippy::needless_return is turned off on purpose: square_verbose shows exactly that form.
#![allow(clippy::needless_return)]

fn square(n: i32) -> i32 {
    n * n // no return needed
}

fn square_verbose(n: i32) -> i32 {
    return n * n; // also works, but idiomatic code has no return
}

fn main() {
    println!("{} {}", square(4), square_verbose(4));
}
