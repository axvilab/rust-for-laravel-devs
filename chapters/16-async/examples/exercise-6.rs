//! Exercise 6: 5 tasks, task i sleeps (6 - i) * 100 ms — the order in a JoinSet and in a Vec

use std::time::{Duration, Instant};
use tokio::task::JoinSet;

async fn job(i: u64) -> u64 {
    tokio::time::sleep(Duration::from_millis((6 - i) * 100)).await;
    i
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    let mut set = JoinSet::new();
    for i in 1..=5 {
        set.spawn(job(i));
    }
    while let Some(res) = set.join_next().await {
        println!("JoinSet: {} after {:?}", res.unwrap(), start.elapsed());
    }
    // 5, 4, 3, 2, 1 — as they finish

    let start = Instant::now();
    let handles: Vec<_> = (1..=5).map(|i| tokio::spawn(job(i))).collect();
    for h in handles {
        println!("Vec:     {} after {:?}", h.await.unwrap(), start.elapsed());
    }
    // 1, 2, 3, 4, 5 — in order: everyone waits for the first one (500 ms), the rest are ready by then
}
