//! Exercise 5: `match &maybe` — one character, and `maybe` stays alive

fn main() {
    let maybe: Option<String> = Some("test".into());
    match &maybe {
        Some(s) => println!("present: {s}"), // s is a &String
        None => println!("absent"),
    }
    println!("{maybe:?}"); // Some("test")
}
