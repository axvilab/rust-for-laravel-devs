//! Структуру з приватним полем не створити літералом ззовні модуля
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0451]: field `password_hash` of struct `User` is private
//! Виправлення: створювати через `User::new(...)` — див. examples/14-visibility.rs
// expected-error: E0451

mod users {
    pub struct User {
        pub id: u64,
        password_hash: String,
    }
}

fn main() {
    let u = users::User {
        id: 1,
        password_hash: "x".into(),
    };
    println!("{}", u.id);
}
