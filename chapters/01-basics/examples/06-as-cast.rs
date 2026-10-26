//! `as` — explicit numeric conversion

fn main() {
    let x: i32 = 5;
    let y: f64 = 2.5;
    let z = x as f64 + y; // ok: converted explicitly
    println!("{z}"); // 7.5
}
