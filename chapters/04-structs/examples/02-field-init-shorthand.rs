//! Скорочення `name` замість `name: name`

struct User {
    id: u64,
    name: String,
    email: String,
    active: bool,
}

fn main() {
    let name = String::from("Віталій");
    let email = String::from("v@example.com");
    let user = User {
        id: 1,
        name,
        email,
        active: true,
    }; // name: name → name
    println!("{} {} {} {}", user.id, user.name, user.email, user.active);
}
