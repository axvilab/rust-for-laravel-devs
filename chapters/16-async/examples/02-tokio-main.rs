//! `#[tokio::main]` — creates the runtime and runs the async main

struct User {
    name: String,
}

async fn fetch_user(id: u64) -> Result<User, String> {
    Ok(User {
        name: format!("user{id}"),
    })
}

#[tokio::main]
async fn main() {
    let user = fetch_user(1).await.unwrap();
    println!("{}", user.name);
}
