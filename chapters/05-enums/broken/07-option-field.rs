//! До поля всередині `Option` не дістатися без перевірки
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0609]: no field `name` on type `Option<User>`
//! Виправлення: `match` / `if let` — див. examples/07-option.rs
// expected-error: E0609

struct User {
    name: String,
}

fn find_user(id: u64) -> Option<User> {
    (id == 1).then(|| User {
        name: "Віталій".into(),
    })
}

fn main() {
    println!("{}", find_user(5).name);
}
