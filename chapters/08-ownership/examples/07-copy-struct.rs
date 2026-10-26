//! `#[derive(Clone, Copy)]` для маленької «числової» структури

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

fn main() {
    let a = Point { x: 1.0, y: 2.0 };
    let b = a; // копія, a живий
    println!("{a:?} {b:?}");
}
