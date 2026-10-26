//! Вправа 1 (зламана версія): використання після move
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0382]: borrow of moved value: `s`
//! Виправлення: див. examples/exercise-1.rs
// expected-error: E0382

fn main() {
    let s = String::from("тест");
    let t = s;
    println!("{s} {t}");
}
