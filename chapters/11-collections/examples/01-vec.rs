//! `Vec` — a list: push, len, [i], get, first/last, pop, insert, remove, contains
// `names` never changes, so clippy suggests an array instead of vec! (useless_vec).
// The point here is the vec! macro itself, so the lint is turned off.
#![allow(clippy::useless_vec)]

fn main() {
    let mut users: Vec<String> = Vec::new();
    users.push("Ivan".to_string());
    users.push("Maria".to_string());

    let names = vec!["Ivan", "Maria", "Oleh"]; // the vec! macro — like [ ... ] in PHP

    println!("{}", names.len()); // 3
    println!("{}", names[0]); // "Ivan" — panics if the index is out of bounds!
    println!("{:?}", names.get(10)); // None — safe access
    println!("{:?}", names.first()); // Some("Ivan")
    println!("{:?}", names.last()); // Some("Oleh")

    let popped = users.pop(); // Option<String> — take the last one
    println!("{popped:?}"); // Some("Maria")
    users.insert(0, "First".into()); // insert at an index
    users.remove(1); // remove at an index
    println!("{}", users.contains(&"Ivan".to_string())); // false — "Ivan" was just removed
    println!("{users:?}"); // ["First"]
}
