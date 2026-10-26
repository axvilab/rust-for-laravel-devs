//! Вправа 5: частота слів — `HashMap` (порядок випадковий) проти `BTreeMap` (стабільний)

use std::collections::{BTreeMap, HashMap};

fn main() {
    let sentence = "кіт бачить пса пес бачить кота кіт тікає";

    let mut freq: HashMap<&str, u32> = HashMap::new();
    for w in sentence.split_whitespace() {
        *freq.entry(w).or_insert(0) += 1;
    }
    println!("HashMap (запусти кілька разів — порядок зміниться):");
    for (w, n) in &freq {
        println!("  {w}: {n}");
    }

    // Той самий API, але ключі завжди відсортовані.
    let mut sorted: BTreeMap<&str, u32> = BTreeMap::new();
    for w in sentence.split_whitespace() {
        *sorted.entry(w).or_insert(0) += 1;
    }
    println!("BTreeMap (порядок стабільний):");
    for (w, n) in &sorted {
        println!("  {w}: {n}");
    }
}
