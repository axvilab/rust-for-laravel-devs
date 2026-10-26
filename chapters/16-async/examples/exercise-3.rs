//! Exercise 3: `std::thread::sleep` inside `join!` — sequential again
//!
//! Why: `join!` runs three futures in ONE task on one thread, polling them
//! in turn. `tokio::time::sleep` yields ("I'm waiting, go do the others"),
//! while `std::thread::sleep` blocks the thread itself — the other futures wait until it wakes up.

use std::time::{Duration, Instant};

async fn slow_blocking(name: &str, ms: u64) -> String {
    std::thread::sleep(Duration::from_millis(ms)); // DON'T do this in async code
    name.to_string()
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    let (a, b, c) = tokio::join!(
        slow_blocking("a", 200),
        slow_blocking("b", 200),
        slow_blocking("c", 200)
    );
    println!("{a}{b}{c} in {:?}", start.elapsed()); // ~600 ms, not 200
}
