//! Exercise 4: `dbg!` in the middle of an expression, and the output order
//!
//! `dbg!` writes to stderr and passes the value on, so the rest of the line stays the same.
//! First you'll see `[examples/exercise-4.rs:..] price * qty as f64 = 599.97`,
//! then the line from `println!`. In a terminal both streams are mixed, but
//! `cargo run ... 2>/dev/null` leaves only the `println!` output.

fn main() {
    let price: f64 = 199.99;
    let qty: u32 = 3;
    let total = dbg!(price * qty as f64);
    println!("total: {total:.2}");
}
