//! Сортування `Vec`: sort, sort_by, sort_by_key, partial_cmp для f64

#[derive(Debug)]
struct User {
    name: String,
    age: u32,
}

fn main() {
    let mut nums = vec![3, 1, 2];
    nums.sort(); // 1 2 3
    println!("{nums:?}");
    nums.sort_by(|a, b| b.cmp(a)); // 3 2 1: cmp повертає Less/Equal/Greater
    println!("{nums:?}");

    let mut users = vec![
        User {
            name: "Олег".into(),
            age: 41,
        },
        User {
            name: "Іван".into(),
            age: 29,
        },
        User {
            name: "Марія".into(),
            age: 35,
        },
    ];
    users.sort_by_key(|u| u.age); // за полем
    println!("{users:?}");
    users.sort_by(|a, b| a.name.cmp(&b.name)); // за рядком
    println!("{users:?}");

    let mut prices = vec![2.5, 1.0];
    prices.sort_by(|a, b| a.partial_cmp(b).unwrap()); // f64 не Ord через NaN
    println!("{prices:?}");
}
