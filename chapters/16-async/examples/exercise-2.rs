//! Exercise 2: the same with `tokio::spawn` and a loop over `handles`
//!
//! `slow` takes a `&str`, and a task must be 'static — so spawn gets
//! string literals (&'static str) or a String via `async move`.

use std::time::{Duration, Instant};

async fn slow(name: &str, ms: u64) -> String {
    tokio::time::sleep(Duration::from_millis(ms)).await;
    name.to_string()
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    let handles: Vec<_> = ["a", "b", "c"]
        .into_iter()
        .map(|name| tokio::spawn(slow(name, 200))) // literals are 'static, allowed
        .collect();
    let mut out = String::new();
    for h in handles {
        out.push_str(&h.await.unwrap());
    }
    println!("spawn: {out} in {:?}", start.elapsed()); // ~200 ms
}
