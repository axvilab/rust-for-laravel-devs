//! A Future does nothing until it's `.await`ed

async fn fetch_user(id: u64) -> u64 {
    println!("  → request for {id} sent");
    id
}

#[tokio::main]
async fn main() {
    let fut = fetch_user(5); // created a Future — the request has NOT been sent yet
    println!("future created, but nothing was printed");
    let id = fut.await; // only now
    println!("got {id}");
    // A bare `fetch_user(5);` without a variable and without .await gets flagged by the
    // compiler with the warning `unused implementer of Future that must be used`.
}
