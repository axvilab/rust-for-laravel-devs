//! `if let` — when you care about one arm

struct User {
    name: String,
}

// A stand-in for a "database lookup": only the user with id 1 exists.
fn find_user(id: u64) -> Option<User> {
    if id == 1 {
        Some(User {
            name: "Vitalii".into(),
        })
    } else {
        None
    }
}

fn main() {
    for id in [1, 2] {
        if let Some(user) = find_user(id) {
            println!("found: {}", user.name);
        } else {
            println!("no such user");
        }
    }
}
