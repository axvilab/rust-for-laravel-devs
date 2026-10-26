//! Copy: numbers are copied, not moved

fn main() {
    let a = 5;
    let b = a;
    println!("{a} {b}"); // ok: 5 5
}
