//! Вправа 5 (зламана версія): `match` без `&` забрав вміст Option
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0382]: borrow of partially moved value: `maybe`
//! Виправлення одним символом: `match &maybe` — див. examples/exercise-5.rs
// expected-error: partially moved

fn main() {
    let maybe: Option<String> = Some("тест".into());
    match maybe {
        Some(s) => println!("є: {s}"),
        None => println!("нема"),
    }
    println!("{maybe:?}");
}
