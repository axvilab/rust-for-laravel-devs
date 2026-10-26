//! `join!` — wait for both at once; `select!` — whoever is first; `timeout`

use std::time::{Duration, Instant};

async fn fetch_user(id: u64) -> String {
    tokio::time::sleep(Duration::from_millis(100)).await;
    format!("user{id}")
}

async fn fetch_orders(id: u64) -> Vec<u64> {
    tokio::time::sleep(Duration::from_millis(100)).await;
    vec![id * 10, id * 10 + 1]
}

async fn fetch_from(source: &str, ms: u64) -> String {
    tokio::time::sleep(Duration::from_millis(ms)).await;
    format!("response from {source}")
}

#[derive(Debug)]
struct Timeout;

async fn fastest() -> Result<String, Timeout> {
    // whoever is first wins, the rest are cancelled
    tokio::select! {
        result = fetch_from("primary", 300) => Ok(result),
        result = fetch_from("replica", 50) => Ok(result),
        _ = tokio::time::sleep(Duration::from_secs(5)) => Err(Timeout),
    }
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    // both at once, wait for both: ~100 ms, not 200
    let (user, orders) = tokio::join!(fetch_user(7), fetch_orders(7));
    println!("{user} {orders:?} in {:?}", start.elapsed());

    println!("{:?}", fastest().await); // replica

    // a timeout without a macro
    let r = tokio::time::timeout(Duration::from_millis(50), fetch_user(1)).await;
    println!("timeout: {r:?}"); // Err(Elapsed(()))
}
