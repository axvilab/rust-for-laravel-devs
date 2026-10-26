//! `Option` замість null: `match` і `if let`

struct User {
    id: u64,
    name: String,
    email: String,
}

impl User {
    fn new(id: u64, name: &str, email: &str) -> Self {
        Self {
            id,
            name: name.into(),
            email: email.into(),
        }
    }
}

fn find_user(id: u64) -> Option<User> {
    if id == 1 {
        Some(User::new(1, "Віталій", "v@example.com"))
    } else {
        None
    }
}

fn main() {
    match find_user(5) {
        Some(user) => println!("{}", user.name),
        None => println!("не знайдено"),
    }

    // або коротше, коли цікавить тільки один випадок:
    if let Some(user) = find_user(1) {
        println!("{} ({}, #{})", user.name, user.email, user.id);
    }
}
