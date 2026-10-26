//! `.sum()` without a type hint
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0283]: type annotations needed
//! Fix: `let total: f64 = ...` or `.sum::<f64>()` — see examples/06-iterator-chain.rs
// expected-error: E0283

fn main() {
    let prices = [100.0, 250.0];
    let total = prices.iter().sum();
    println!("{total}");
}
