//! Вправа 6: 5 задач, задача i спить (6 - i) * 100 мс — порядок у JoinSet і у Vec

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
        println!("JoinSet: {} через {:?}", res.unwrap(), start.elapsed());
    }
    // 5, 4, 3, 2, 1 — у міру готовності

    let start = Instant::now();
    let handles: Vec<_> = (1..=5).map(|i| tokio::spawn(job(i))).collect();
    for h in handles {
        println!("Vec:     {} через {:?}", h.await.unwrap(), start.elapsed());
    }
    // 1, 2, 3, 4, 5 — по порядку: усі чекають на першу (500 мс), решта вже готові
}
