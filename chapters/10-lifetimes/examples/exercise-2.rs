//! Exercise 2: what the compiler suggests if you remove the lifetimes
//!
//! broken/exercise-2.rs gives E0106 with a ready-made line in `help:` —
//! `fn pick<'a>(cond: bool, a: &'a str, b: &'a str) -> &'a str`.
//! That's exactly what's inserted here. Why the compiler doesn't infer it: there are
//! two reference parameters, so the "one input → output gets its lifetime" rule doesn't apply.

fn pick<'a>(cond: bool, a: &'a str, b: &'a str) -> &'a str {
    if cond { a } else { b }
}

fn main() {
    println!("{}", pick(false, "yes", "no"));
}
