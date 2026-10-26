//! Exercise 5: what `#[derive(Debug, Clone)]` expands to (as `cargo expand` would show)
//!
//! `cargo install cargo-expand` and `cargo expand --example exercise-5` show
//! almost literally the implementations below. Here they're written by hand for `Point`,
//! while `DerivedPoint` gets the same via derive — the output is identical.

use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

// ≈ what #[derive(Debug)] generates
impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Point")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

// ≈ what #[derive(Clone)] generates: a clone of every field
impl Clone for Point {
    fn clone(&self) -> Self {
        Point {
            x: Clone::clone(&self.x),
            y: Clone::clone(&self.y),
        }
    }
}

#[derive(Debug, Clone)]
struct DerivedPoint {
    x: i32,
    y: i32,
}

fn main() {
    let a = Point { x: 1, y: 2 };
    let b = DerivedPoint { x: 1, y: 2 };
    println!("{:?}", a.clone()); // Point { x: 1, y: 2 }
    println!("{:?}", b.clone()); // DerivedPoint { x: 1, y: 2 }
}
