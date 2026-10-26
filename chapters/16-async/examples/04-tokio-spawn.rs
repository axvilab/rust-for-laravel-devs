//! `tokio::spawn(async move { ... })` — дешева задача, аналог `thread::spawn`

use std::time::Duration;

struct User {
    email: String,
}

async fn send_email(user: User) -> Result<String, String> {
    tokio::time::sleep(Duration::from_millis(50)).await;
    Ok(format!("лист надіслано на {}", user.email))
}

#[tokio::main]
async fn main() {
    let user = User {
        email: "v@example.com".into(),
    };
    let handle = tokio::spawn(async move { send_email(user).await });

    // робимо щось інше...
    println!("головна задача працює далі");

    let result = handle.await.unwrap(); // JoinError → unwrap; всередині — наш Result
    println!("{result:?}");
}
