//! Не блокуй рантайм: `tokio::time::sleep`, `tokio::fs`, `spawn_blocking` для CPU-роботи

use std::time::{Duration, Instant};

fn expensive_hash(password: &str) -> u64 {
    // імітація argon2: ~100 мс чистого CPU
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
    tokio::time::sleep(Duration::from_millis(10)).await; // спати — так, не std::thread::sleep

    let cargo_toml = tokio::fs::read_to_string("Cargo.toml").await; // файли — tokio::fs
    println!("Cargo.toml прочитано: {}", cargo_toml.is_ok());

    // важке обчислення — в окремий пул потоків
    let password = String::from("secret");
    let hash = tokio::task::spawn_blocking(move || expensive_hash(&password))
        .await
        .unwrap();
    println!("хеш: {hash:x}");
}
