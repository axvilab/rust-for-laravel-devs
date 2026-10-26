//! `let else` — return early if the pattern doesn't match

struct User {
    name: String,
}

fn find_user(id: u64) -> Option<User> {
    (id == 1).then(|| User {
        name: "Vitalii".into(),
    })
}

// Like a handler: either a user or an error.
fn greet(id: u64) -> Result<String, &'static str> {
    let Some(user) = find_user(id) else {
        return Err("user not found");
    };
    // here user definitely exists
    Ok(format!("Hello, {}", user.name))
}

fn main() {
    println!("{:?}", greet(1)); // Ok("Hello, Vitalii")
    println!("{:?}", greet(2)); // Err("user not found")
}
