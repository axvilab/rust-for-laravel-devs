//! A thousand concurrent tasks; `h.await??` — two levels of errors

use std::error::Error;
use std::time::{Duration, Instant};

#[derive(Debug)]
struct User {
    id: u64,
}

async fn fetch_user(id: u64) -> Result<User, String> {
    tokio::time::sleep(Duration::from_millis(100)).await;
    Ok(User { id })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let ids: Vec<u64> = (1..=1000).collect();
    let start = Instant::now();

    let handles: Vec<_> = ids
        .into_iter()
        .map(|id| tokio::spawn(fetch_user(id)))
        .collect();

    let mut sum = 0;
    for h in handles {
        let user = h.await??; // the first ? — the task didn't fail, the second — fetch didn't return Err
        sum += user.id;
    }
    // 1000 "requests" of 100 ms each — ~100 ms in total, not 100 seconds
    println!("sum of ids {sum} in {:?}", start.elapsed());
    Ok(())
}
