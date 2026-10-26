//! `Arc::clone(&x)` for a value that isn't in an `Arc` doesn't compile — and that's a plus
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0308]: mismatched types (expected `&Arc<_>`, found `&Config`)
//! Fix: `Arc::new(Config { .. })` — see examples/09-arc-clone.rs
// expected-error: E0308

use std::sync::Arc;

#[derive(Clone)]
struct Config {
    port: u16,
}

fn main() {
    let config = Config { port: 3000 };
    let copy = Arc::clone(&config);
    println!("{}", copy.port);
}
