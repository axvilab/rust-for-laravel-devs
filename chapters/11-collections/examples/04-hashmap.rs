//! `HashMap`: insert, get → Option, entry().or_insert(), ітерація, remove

use std::collections::HashMap;

fn main() {
    let mut stock: HashMap<String, u32> = HashMap::new();
    stock.insert("ноутбук".to_string(), 5);
    stock.insert("миша".to_string(), 40);

    // читання — завжди Option, бо ключа може не бути
    match stock.get("ноутбук") {
        Some(qty) => println!("є {qty} шт"),
        None => println!("нема"),
    }

    // «візьми або створи» — замість isset + присвоєння
    *stock.entry("клавіатура".to_string()).or_insert(0) += 10;

    // ітерація: порядок випадковий і змінюється між запусками
    for (name, qty) in &stock {
        println!("{name}: {qty}");
    }

    stock.remove("миша");
    println!("{}", stock.contains_key("миша")); // false
}
