//! Посилання на локальну змінну не можна повернути
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0106]: missing lifetime specifier
//!   (з двома `help:` — додати `'static` (пастка!) або повернути `String`)
//! Виправлення: повертати володіння — див. examples/09-return-owned.rs
// expected-error: E0106

fn longest_name() -> &str {
    let name = String::from("Віталій");
    &name // ПОМИЛКА: повертаємо посилання на те, що зараз знищиться
}

fn main() {
    println!("{}", longest_name());
}
