//! `#[derive(Clone, Copy)]` for a small "numeric" struct

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

fn main() {
    let a = Point { x: 1.0, y: 2.0 };
    let b = a; // a copy, a is still alive
    println!("{a:?} {b:?}");
}
