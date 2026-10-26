//! Declaring a struct and building it with a literal with named fields

struct User {
    id: u64,
    name: String,
    email: String,
    active: bool,
}

fn main() {
    let user = User {
        id: 1,
        name: String::from("Vitalii"),
        email: String::from("v@example.com"),
        active: true,
    };

    println!("{}", user.name);
}
