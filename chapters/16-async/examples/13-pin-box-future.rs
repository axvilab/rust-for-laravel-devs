//! `Pin<Box<dyn Future<Output = T> + Send>>` — коли гілки повертають різні futures

use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

type BoxFuture<T> = Pin<Box<dyn Future<Output = T> + Send>>;

fn load(from_cache: bool) -> BoxFuture<String> {
    if from_cache {
        Box::pin(async { "з кешу".to_string() }) // одна future
    } else {
        Box::pin(async {
            // зовсім інша future (інший анонімний тип) — без Box::pin не скомпілювалось би
            tokio::time::sleep(Duration::from_millis(20)).await;
            "з бази".to_string()
        })
    }
}

#[tokio::main]
async fn main() {
    println!("{} / {}", load(true).await, load(false).await);
}
