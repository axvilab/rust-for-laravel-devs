//! `sort_by_key` doesn't accept a reference key
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error: lifetime may not live long enough
//! Fix: `sort_by(|a, b| a.name.cmp(&b.name))` or
//!   `sort_by_key(|u| u.name.clone())` — see examples/13-sort-by-name.rs
// expected-error: lifetime may not live long enough

struct User {
    name: String,
}

fn main() {
    let mut users = vec![User { name: "b".into() }, User { name: "a".into() }];
    users.sort_by_key(|u| &u.name);
}
