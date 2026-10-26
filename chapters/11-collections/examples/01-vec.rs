//! `Vec` — список: push, len, [i], get, first/last, pop, insert, remove, contains
// `names` ніде не змінюється, тому clippy радить масив замість vec! (useless_vec).
// Тут показуємо саме макрос vec!, тож лінт вимкнено.
#![allow(clippy::useless_vec)]

fn main() {
    let mut users: Vec<String> = Vec::new();
    users.push("Іван".to_string());
    users.push("Марія".to_string());

    let names = vec!["Іван", "Марія", "Олег"]; // макрос vec! — як [ ... ] у PHP

    println!("{}", names.len()); // 3
    println!("{}", names[0]); // "Іван" — паніка, якщо індекс за межами!
    println!("{:?}", names.get(10)); // None — безпечний доступ
    println!("{:?}", names.first()); // Some("Іван")
    println!("{:?}", names.last()); // Some("Олег")

    let popped = users.pop(); // Option<String> — забрати останній
    println!("{popped:?}"); // Some("Марія")
    users.insert(0, "Перший".into()); // вставити за індексом
    users.remove(1); // видалити за індексом
    println!("{}", users.contains(&"Іван".to_string())); // false — «Іван» щойно видалено
    println!("{users:?}"); // ["Перший"]
}
