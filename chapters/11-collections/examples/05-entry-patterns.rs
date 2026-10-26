//! Усі прийоми `entry()`: підрахунок, групування, and_modify, or_insert_with

use std::collections::HashMap;

fn load_name(id: u64) -> String {
    println!("(дорогий запит для {id})");
    format!("користувач #{id}")
}

fn main() {
    let words = ["кіт", "пес", "кіт", "папуга"];

    // підрахунок
    let mut counts: HashMap<&str, u32> = HashMap::new();
    for w in words {
        *counts.entry(w).or_insert(0) += 1;
    }

    // групування, як groupBy: or_default() створить порожній Vec
    let mut by_letter: HashMap<char, Vec<&str>> = HashMap::new();
    for w in words {
        let first = w.chars().next().unwrap();
        by_letter.entry(first).or_default().push(w);
    }

    // «якщо є — онови, якщо нема — встав»
    counts.entry("рибка").and_modify(|n| *n += 1).or_insert(1);

    // дороге значення — обчислюється лише якщо ключа нема
    let mut cache: HashMap<u64, String> = HashMap::new();
    let name = cache.entry(42).or_insert_with(|| load_name(42));
    println!("{name}");
    let again = cache.entry(42).or_insert_with(|| load_name(42)); // «дорогий запит» не друкується
    println!("{again}");

    println!("{counts:?}");
    println!("{by_letter:?}");
}
