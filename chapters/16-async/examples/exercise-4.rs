//! Exercise 4: `select!` with a 100 ms timeout for a function that sleeps 500 ms

use std::time::Duration;

async fn slow() -> &'static str {
    tokio::time::sleep(Duration::from_millis(500)).await;
    "done"
}

#[tokio::main]
async fn main() {
    tokio::select! {
        r = slow() => println!("made it: {r}"),
        _ = tokio::time::sleep(Duration::from_millis(100)) => println!("timeout after 100 ms"),
    }
    // the same without the macro
    match tokio::time::timeout(Duration::from_millis(100), slow()).await {
        Ok(r) => println!("made it: {r}"),
        Err(_) => println!("timeout(): Elapsed"),
    }
}
