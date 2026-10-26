//! `async fn` повертає Future; `.await` чекає; `.await?` — дочекатися і прокинути помилку

use std::time::Duration;

#[derive(Debug)]
struct User {
    id: u64,
    name: String,
}

#[derive(Debug)]
struct DbError(String);

#[derive(Debug)]
struct AppError(String);

impl From<DbError> for AppError {
    fn from(e: DbError) -> Self {
        AppError(format!("db: {}", e.0))
    }
}

// «База» — заглушка: справжній запит у проєкті orders-api.
async fn fetch_user(id: u64) -> Result<User, DbError> {
    tokio::time::sleep(Duration::from_millis(20)).await; // чекаємо «мережу»
    if id == 0 {
        return Err(DbError("not found".into()));
    }
    Ok(User {
        id,
        name: format!("user{id}"),
    })
}

async fn handler(id: u64) -> Result<String, AppError> {
    let user = fetch_user(id).await?; // дочекатися, потім ? з розділу про помилки
    Ok(format!("{{\"id\":{},\"name\":\"{}\"}}", user.id, user.name))
}

#[tokio::main]
async fn main() {
    println!("{:?}", handler(1).await);
    println!("{:?}", handler(0).await);
}
