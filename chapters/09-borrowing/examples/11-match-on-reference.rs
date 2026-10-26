//! `match &x` дивиться, `match x` забирає вміст

fn main() {
    let name: Option<String> = Some("Віталій".to_string());

    match &name {
        // дивимось, не забираємо
        Some(n) => println!("є: {n}"), // n тут — &String
        None => println!("нема"),
    }
    println!("{name:?}"); // name живий

    // У книзі тут `match` з порожньою гілкою `None => {}`. Clippy радить для
    // такого `if let` (clippy::single_match); форму `match` залишено навмисно.
    #[allow(clippy::single_match)]
    match name {
        // без & — забираємо вміст
        Some(n) => println!("забрали: {n}"), // n — String, переїхав
        None => {}
    }
    // println!("{name:?}");            // ПОМИЛКА: borrow of partially moved value
}
