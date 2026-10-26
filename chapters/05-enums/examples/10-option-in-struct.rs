//! `Option` in fields — like nullable columns

use std::time::SystemTime;

// The book has `deleted_at: Option<DateTime>` (a type from the chrono crate). To keep
// the example dependency-free it uses the standard `SystemTime`; with chrono it would be
// `Option<chrono::DateTime<chrono::Utc>>`.
#[derive(Debug)]
struct User {
    id: u64,
    name: String,
    phone: Option<String>,          // may be missing
    deleted_at: Option<SystemTime>, // soft delete
}

fn main() {
    let u = User {
        id: 1,
        name: "Vitalii".into(),
        phone: None,
        deleted_at: None,
    };
    match &u.phone {
        Some(p) => println!("phone: {p}"),
        None => println!("{} has no phone", u.name),
    }
    println!("deleted: {}", u.deleted_at.is_some());
}
