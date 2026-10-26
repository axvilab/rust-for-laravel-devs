//! `&str` і `String`: літерал, власний рядок, `format!`

fn main() {
    let greeting = "Привіт"; // &str, зашитий у бінарник
    let mut name = String::from("Віталій"); // String, у купі
    // У книзі тут `push_str("!")`; clippy (single_char_add_str) радить для одного
    // символу `push('!')`. Щоб показати саме push_str, дописуємо кілька символів.
    name.push_str(" з Дніпра"); // можна міняти
    let full = format!("{greeting}, {name}"); // String
    println!("{full}");
}
