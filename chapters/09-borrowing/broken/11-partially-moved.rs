//! `match` без `&` забрав рядок з Option — `name` більше не цілий
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0382]: borrow of partially moved value: `name`
//! Виправлення: `match &name` — див. examples/11-match-on-reference.rs
// expected-error: partially moved

fn main() {
    let name: Option<String> = Some("Віталій".to_string());
    if let Some(n) = name {
        println!("забрали: {n}");
    }
    println!("{name:?}");
}
