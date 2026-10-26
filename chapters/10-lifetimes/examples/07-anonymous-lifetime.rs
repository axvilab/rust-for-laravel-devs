//! `'_` — the anonymous lifetime: "something is borrowed here, details don't matter"

use std::fmt;

struct Money(i64);

impl fmt::Display for Money {
    // Formatter<'_> — Formatter has a lifetime, but there's no need to name it
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} UAH", self.0)
    }
}

struct Excerpt<'a> {
    part: &'a str,
}

// impl for a struct with a lifetime: '_ instead of a made-up name
impl Excerpt<'_> {
    fn len(&self) -> usize {
        self.part.chars().count()
    }
}

fn main() {
    println!("{}", Money(1500));
    let text = String::from("A chapter about lifetimes");
    let e = Excerpt { part: &text };
    println!("{}", e.len());
}
