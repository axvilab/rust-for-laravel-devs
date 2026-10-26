//! `if let ... && condition` — let chains (edition 2024, Rust 1.88+)

struct User {
    name: String,
    active: bool,
}

fn find_user(id: u64) -> Option<User> {
    match id {
        1 => Some(User {
            name: "Vitalii".into(),
            active: true,
        }),
        2 => Some(User {
            name: "Blocked".into(),
            active: false,
        }),
        _ => None,
    }
}

fn main() {
    for id in [1, 2, 3] {
        if let Some(user) = find_user(id)
            && user.active
        {
            println!("welcome, {}", user.name);
        } else {
            println!("id {id}: missing or inactive");
        }
    }
}
