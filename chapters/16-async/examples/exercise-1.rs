//! Exercise 1: three `slow` calls in sequence (~600 ms) vs `tokio::join!` (~200 ms)

use std::time::{Duration, Instant};

async fn slow(name: &str, ms: u64) -> String {
    tokio::time::sleep(Duration::from_millis(ms)).await;
    name.to_string()
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    let a = slow("a", 200).await;
    let b = slow("b", 200).await;
    let c = slow("c", 200).await;
    println!("sequential: {a}{b}{c} in {:?}", start.elapsed()); // ~600 ms

    let start = Instant::now();
    let (a, b, c) = tokio::join!(slow("a", 200), slow("b", 200), slow("c", 200));
    println!("join!:      {a}{b}{c} in {:?}", start.elapsed()); // ~200 ms
}
