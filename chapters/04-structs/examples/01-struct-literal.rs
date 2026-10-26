//! Оголошення структури і створення літералом з іменованими полями

struct User {
    id: u64,
    name: String,
    email: String,
    active: bool,
}

fn main() {
    let user = User {
        id: 1,
        name: String::from("Віталій"),
        email: String::from("v@example.com"),
        active: true,
    };

    println!("{}", user.name);
}
