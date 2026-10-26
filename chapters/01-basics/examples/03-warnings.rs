//! Compiler warnings: an unused variable and the `_` prefix
//!
//! This file has `#![allow(unused_variables)]` because CI builds with `-D warnings`.
//! Remove the `allow` line and run `cargo build -p ch01-basics --example 03-warnings` —
//! you'll see `warning: unused variable: x` and the hint `help: ... prefix it with an underscore`.
#![allow(unused_variables)]

fn main() {
    let x = 5; // warning: unused variable: `x`
    let _y = 6; // silence: "I know I'm not using it"
}
