//! `sort_by_key` не приймає ключ-посилання
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error: lifetime may not live long enough
//! Виправлення: `sort_by(|a, b| a.name.cmp(&b.name))` або
//!   `sort_by_key(|u| u.name.clone())` — див. examples/13-sort-by-name.rs
// expected-error: lifetime may not live long enough

struct User {
    name: String,
}

fn main() {
    let mut users = vec![User { name: "b".into() }, User { name: "a".into() }];
    users.sort_by_key(|u| &u.name);
}
