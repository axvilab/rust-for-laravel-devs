//! `collect()` into a Vec, HashMap, HashSet, String; the turbofish `::<Vec<_>>`

use std::collections::{HashMap, HashSet};

struct User {
    id: u64,
    name: String,
}

fn main() {
    let users = [
        User {
            id: 1,
            name: "Ivan".into(),
        },
        User {
            id: 2,
            name: "Maria".into(),
        },
    ];
    let words = ["cat", "dog", "cat"];
    let parts = [String::from("Rust"), String::from("+"), String::from("PHP")];

    let names: Vec<String> = users.iter().map(|u| u.name.clone()).collect();
    let by_id: HashMap<u64, &User> = users.iter().map(|u| (u.id, u)).collect();
    let unique: HashSet<&str> = words.iter().copied().collect(); // copied — see below
    let joined: String = parts.iter().map(|s| s.as_str()).collect(); // concatenation

    // or the turbofish if the variable type isn't given:
    let refs = users.iter().map(|u| &u.name).collect::<Vec<_>>();

    println!(
        "{names:?} {} {} {joined} {refs:?}",
        by_id[&2].name,
        unique.len()
    );
}
