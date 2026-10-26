//! A Copy struct is a separate copy, not "the same object" as in PHP

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

fn shift(mut p: Point) {
    p.x += 10; // we change the COPY
    println!("inside shift: {p:?}");
}

fn main() {
    let a = Point { x: 1, y: 2 };
    let mut b = a; // a COPY, not "the same object"
    b.x = 100;
    println!("{a:?}"); // Point { x: 1, y: 2 } — a didn't change
    println!("{b:?}"); // Point { x: 100, y: 2 }

    shift(a);
    println!("{a:?}"); // still x: 1 — the function got a copy
}
