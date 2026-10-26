//! Exercise 5: word frequency — `HashMap` (random order) vs `BTreeMap` (stable)

use std::collections::{BTreeMap, HashMap};

fn main() {
    let sentence = "the cat sees the dog the dog sees a cat the cat runs";

    let mut freq: HashMap<&str, u32> = HashMap::new();
    for w in sentence.split_whitespace() {
        *freq.entry(w).or_insert(0) += 1;
    }
    println!("HashMap (run it a few times — the order changes):");
    for (w, n) in &freq {
        println!("  {w}: {n}");
    }

    // The same API, but the keys are always sorted.
    let mut sorted: BTreeMap<&str, u32> = BTreeMap::new();
    for w in sentence.split_whitespace() {
        *sorted.entry(w).or_insert(0) += 1;
    }
    println!("BTreeMap (stable order):");
    for (w, n) in &sorted {
        println!("  {w}: {n}");
    }
}
