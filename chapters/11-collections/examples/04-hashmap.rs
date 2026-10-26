//! `HashMap`: insert, get → Option, entry().or_insert(), iteration, remove

use std::collections::HashMap;

fn main() {
    let mut stock: HashMap<String, u32> = HashMap::new();
    stock.insert("laptop".to_string(), 5);
    stock.insert("mouse".to_string(), 40);

    // reading always gives an Option, because the key may be missing
    match stock.get("laptop") {
        Some(qty) => println!("{qty} in stock"),
        None => println!("none"),
    }

    // "get or create" — instead of isset + assignment
    *stock.entry("keyboard".to_string()).or_insert(0) += 10;

    // iteration: the order is random and changes between runs
    for (name, qty) in &stock {
        println!("{name}: {qty}");
    }

    stock.remove("mouse");
    println!("{}", stock.contains_key("mouse")); // false
}
