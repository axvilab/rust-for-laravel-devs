//! `Arc::clone(&x)` instead of `x.clone()`: it's visibly a cheap "pass", not a copy

use std::sync::Arc;

struct Config {
    port: u16,
}

fn main() {
    let config = Arc::new(Config { port: 3000 });
    let a = Arc::clone(&config); // one more owner, Config isn't copied
    let b = config.clone(); // the same, but a reader may think of an expensive copy
    println!("{} {} {}", a.port, b.port, Arc::strong_count(&config)); // 3000 3000 3
}
