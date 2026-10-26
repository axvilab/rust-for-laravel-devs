//! A `_` at the top swallows everything: the arms below are unreachable
//!
//! The compiler warns `unreachable pattern` ("no value can reach this").
//! The `allow` is here only for CI (`-D warnings`); remove it to see the warning.
#![allow(unreachable_patterns)]

fn main() {
    let status = 404;
    let label = match status {
        _ => "unknown",
        404 => "not found", // nobody will ever get here
    };
    println!("{label}"); // unknown
}
