//! `Option` у полях — аналог nullable-колонок

use std::time::SystemTime;

// У книзі `deleted_at: Option<DateTime>` (тип з крейту chrono). Щоб приклад
// не тягнув залежностей, тут стандартний `SystemTime`; з chrono це було б
// `Option<chrono::DateTime<chrono::Utc>>`.
#[derive(Debug)]
struct User {
    id: u64,
    name: String,
    phone: Option<String>,          // може бути відсутній
    deleted_at: Option<SystemTime>, // soft delete
}

fn main() {
    let u = User {
        id: 1,
        name: "Віталій".into(),
        phone: None,
        deleted_at: None,
    };
    match &u.phone {
        Some(p) => println!("телефон: {p}"),
        None => println!("{} без телефону", u.name),
    }
    println!("видалений: {}", u.deleted_at.is_some());
}
