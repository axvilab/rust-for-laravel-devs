//! `-> impl Iterator<Item = u32>`: a type you can't write by hand

fn evens(limit: u32) -> impl Iterator<Item = u32> {
    // Not `n % 2 == 0`: for unsigned numbers clippy suggests
    // `n.is_multiple_of(2)` (manual_is_multiple_of, Rust 1.87+).
    (0..limit).filter(|n| n.is_multiple_of(2))
}

fn main() {
    let v: Vec<u32> = evens(10).collect();
    println!("{v:?}"); // [0, 2, 4, 6, 8]
}
