//! `tokio::spawn(async move { ... })` — a cheap task, like `thread::spawn`

use std::time::Duration;

struct User {
    email: String,
}

async fn send_email(user: User) -> Result<String, String> {
    tokio::time::sleep(Duration::from_millis(50)).await;
    Ok(format!("email sent to {}", user.email))
}

#[tokio::main]
async fn main() {
    let user = User {
        email: "v@example.com".into(),
    };
    let handle = tokio::spawn(async move { send_email(user).await });

    // doing something else...
    println!("the main task keeps working");

    let result = handle.await.unwrap(); // JoinError → unwrap; inside is our Result
    println!("{result:?}");
}
