//! All the `entry()` tricks: counting, grouping, and_modify, or_insert_with

use std::collections::HashMap;

fn load_name(id: u64) -> String {
    println!("(an expensive lookup for {id})");
    format!("user #{id}")
}

fn main() {
    let words = ["cat", "dog", "cat", "parrot"];

    // counting
    let mut counts: HashMap<&str, u32> = HashMap::new();
    for w in words {
        *counts.entry(w).or_insert(0) += 1;
    }

    // grouping, like groupBy: or_default() creates an empty Vec
    let mut by_letter: HashMap<char, Vec<&str>> = HashMap::new();
    for w in words {
        let first = w.chars().next().unwrap();
        by_letter.entry(first).or_default().push(w);
    }

    // "update if present, insert if not"
    counts.entry("fish").and_modify(|n| *n += 1).or_insert(1);

    // an expensive value — computed only if the key is missing
    let mut cache: HashMap<u64, String> = HashMap::new();
    let name = cache.entry(42).or_insert_with(|| load_name(42));
    println!("{name}");
    let again = cache.entry(42).or_insert_with(|| load_name(42)); // "expensive lookup" isn't printed
    println!("{again}");

    println!("{counts:?}");
    println!("{by_letter:?}");
}
