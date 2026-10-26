//! `join!` — чекати обидва одночасно; `select!` — хто перший; `timeout`

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
    format!("відповідь від {source}")
}

#[derive(Debug)]
struct Timeout;

async fn fastest() -> Result<String, Timeout> {
    // хто перший — того і беремо, решту скасовуємо
    tokio::select! {
        result = fetch_from("primary", 300) => Ok(result),
        result = fetch_from("replica", 50) => Ok(result),
        _ = tokio::time::sleep(Duration::from_secs(5)) => Err(Timeout),
    }
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    // обидва одночасно, чекаємо обох: ~100 мс, а не 200
    let (user, orders) = tokio::join!(fetch_user(7), fetch_orders(7));
    println!("{user} {orders:?} за {:?}", start.elapsed());

    println!("{:?}", fastest().await); // replica

    // таймаут без макроса
    let r = tokio::time::timeout(Duration::from_millis(50), fetch_user(1)).await;
    println!("timeout: {r:?}"); // Err(Elapsed(()))
}
