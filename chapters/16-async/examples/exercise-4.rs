//! Вправа 4: `select!` з таймаутом 100 мс для функції, що спить 500 мс

use std::time::Duration;

async fn slow() -> &'static str {
    tokio::time::sleep(Duration::from_millis(500)).await;
    "готово"
}

#[tokio::main]
async fn main() {
    tokio::select! {
        r = slow() => println!("встигли: {r}"),
        _ = tokio::time::sleep(Duration::from_millis(100)) => println!("таймаут 100 мс"),
    }
    // те саме без макроса
    match tokio::time::timeout(Duration::from_millis(100), slow()).await {
        Ok(r) => println!("встигли: {r}"),
        Err(_) => println!("timeout(): Elapsed"),
    }
}
