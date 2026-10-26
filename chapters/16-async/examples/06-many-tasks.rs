//! Тисяча паралельних задач; `h.await??` — два рівні помилок

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
        let user = h.await??; // перший ? — задача не впала, другий — fetch не повернув Err
        sum += user.id;
    }
    // 1000 «запитів» по 100 мс — разом ~100 мс, а не 100 секунд
    println!("сума id {sum} за {:?}", start.elapsed());
    Ok(())
}
