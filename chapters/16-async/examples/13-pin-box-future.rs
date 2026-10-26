//! `Pin<Box<dyn Future<Output = T> + Send>>` — when branches return different futures

use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

type BoxFuture<T> = Pin<Box<dyn Future<Output = T> + Send>>;

fn load(from_cache: bool) -> BoxFuture<String> {
    if from_cache {
        Box::pin(async { "from the cache".to_string() }) // one future
    } else {
        Box::pin(async {
            // a completely different future (another anonymous type) — wouldn't compile without Box::pin
            tokio::time::sleep(Duration::from_millis(20)).await;
            "from the database".to_string()
        })
    }
}

#[tokio::main]
async fn main() {
    println!("{} / {}", load(true).await, load(false).await);
}
