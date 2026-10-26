//! Поле `&str` у структурі без лайфтайму не компілюється
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0106]: missing lifetime specifier
//! Виправлення: у полях структури — `String` (див. examples/03-string-conversions.rs)
// expected-error: E0106

struct User {
    name: &str,
}

fn main() {
    let u = User { name: "Віталій" };
    println!("{}", u.name);
}
