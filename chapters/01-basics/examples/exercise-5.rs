//! Exercise 5: `checked_add` and `saturating_add` for u8
//!
//! The compiler rejects a plain `stock + 10` because it sees the overflow already
//! at build time: `error: this arithmetic operation will overflow` (see broken/exercise-5.rs).

fn main() {
    let stock: u8 = 250;
    println!("{:?}", stock.checked_add(10)); // None — 260 doesn't fit into u8
    println!("{:?}", stock.saturating_add(10)); // 255 — clamped to the maximum
    println!("{:?}", stock.checked_add(5)); // Some(255) — fits
}
