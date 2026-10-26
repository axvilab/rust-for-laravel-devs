//! Copy: числа копіюються, а не переїжджають

fn main() {
    let a = 5;
    let b = a;
    println!("{a} {b}"); // ok: 5 5
}
