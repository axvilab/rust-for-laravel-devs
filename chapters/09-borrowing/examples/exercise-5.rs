//! Вправа 5: `match &maybe` — один символ, і `maybe` лишається живим

fn main() {
    let maybe: Option<String> = Some("тест".into());
    match &maybe {
        Some(s) => println!("є: {s}"), // s — &String
        None => println!("нема"),
    }
    println!("{maybe:?}"); // Some("тест")
}
