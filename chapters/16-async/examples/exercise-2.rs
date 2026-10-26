//! Вправа 2: те саме через `tokio::spawn` і цикл по `handles`
//!
//! `slow` приймає `&str`, а задача має бути 'static — тому в spawn ідуть
//! рядки-літерали (&'static str) або String через `async move`.

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
        .map(|name| tokio::spawn(slow(name, 200))) // літерали — 'static, можна
        .collect();
    let mut out = String::new();
    for h in handles {
        out.push_str(&h.await.unwrap());
    }
    println!("spawn: {out} за {:?}", start.elapsed()); // ~200 мс
}
