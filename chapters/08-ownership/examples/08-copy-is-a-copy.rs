//! Copy-структура — окрема копія, а не «той самий об'єкт», як у PHP

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

fn shift(mut p: Point) {
    p.x += 10; // змінюємо КОПІЮ
    println!("усередині shift: {p:?}");
}

fn main() {
    let a = Point { x: 1, y: 2 };
    let mut b = a; // КОПІЯ, а не «той самий об'єкт»
    b.x = 100;
    println!("{a:?}"); // Point { x: 1, y: 2 } — a не змінився
    println!("{b:?}"); // Point { x: 100, y: 2 }

    shift(a);
    println!("{a:?}"); // досі x: 1 — функція отримала копію
}
