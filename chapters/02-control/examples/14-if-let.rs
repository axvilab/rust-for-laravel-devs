//! `if let` — коли цікавить одна гілка

struct User {
    name: String,
}

// Заглушка «пошуку в базі»: є тільки користувач з id 1.
fn find_user(id: u64) -> Option<User> {
    if id == 1 {
        Some(User {
            name: "Віталій".into(),
        })
    } else {
        None
    }
}

fn main() {
    for id in [1, 2] {
        if let Some(user) = find_user(id) {
            println!("знайдено: {}", user.name);
        } else {
            println!("нема такого");
        }
    }
}
