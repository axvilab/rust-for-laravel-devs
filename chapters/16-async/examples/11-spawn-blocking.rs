//! Don't block the runtime: `tokio::time::sleep`, `tokio::fs`, `spawn_blocking` for CPU work

use std::time::{Duration, Instant};

fn expensive_hash(password: &str) -> u64 {
    // an argon2 imitation: ~100 ms of pure CPU
    let start = Instant::now();
    let mut h: u64 = 1469598103934665603;
    while start.elapsed() < Duration::from_millis(100) {
        for b in password.bytes() {
            h = (h ^ u64::from(b)).wrapping_mul(1099511628211);
        }
    }
    h
}

#[tokio::main]
async fn main() {
    tokio::time::sleep(Duration::from_millis(10)).await; // sleep like this, not std::thread::sleep

    let cargo_toml = tokio::fs::read_to_string("Cargo.toml").await; // files — tokio::fs
    println!("Cargo.toml read: {}", cargo_toml.is_ok());

    // heavy computation — into a separate thread pool
    let password = String::from("secret");
    let hash = tokio::task::spawn_blocking(move || expensive_hash(&password))
        .await
        .unwrap();
    println!("hash: {hash:x}");
}
