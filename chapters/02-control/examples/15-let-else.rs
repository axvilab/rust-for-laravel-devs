//! `let else` — ранній вихід, якщо шаблон не збігся

struct User {
    name: String,
}

fn find_user(id: u64) -> Option<User> {
    (id == 1).then(|| User {
        name: "Віталій".into(),
    })
}

// Схоже на хендлер: або користувач, або помилка.
fn greet(id: u64) -> Result<String, &'static str> {
    let Some(user) = find_user(id) else {
        return Err("користувача не знайдено");
    };
    // тут user вже точно є
    Ok(format!("Привіт, {}", user.name))
}

fn main() {
    println!("{:?}", greet(1)); // Ok("Привіт, Віталій")
    println!("{:?}", greet(2)); // Err("користувача не знайдено")
}
