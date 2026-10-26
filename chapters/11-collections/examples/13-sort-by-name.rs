//! Сортування за рядковим полем: `sort_by` (без копій) або `sort_by_key` з `clone()`

#[derive(Debug)]
struct User {
    name: String,
}

fn main() {
    let mut users = vec![
        User {
            name: "Олег".into(),
        },
        User {
            name: "Іван".into(),
        },
    ];
    users.sort_by(|a, b| a.name.cmp(&b.name)); // без копій
    println!("{users:?}");
    users.sort_by_key(|u| std::cmp::Reverse(u.name.clone())); // клон на кожне порівняння
    println!("{users:?}");
}
