//! Sorting by a string field: `sort_by` (no copies) or `sort_by_key` with `clone()`

#[derive(Debug)]
struct User {
    name: String,
}

fn main() {
    let mut users = vec![
        User {
            name: "Oleh".into(),
        },
        User {
            name: "Ivan".into(),
        },
    ];
    users.sort_by(|a, b| a.name.cmp(&b.name)); // no copies
    println!("{users:?}");
    users.sort_by_key(|u| std::cmp::Reverse(u.name.clone())); // a clone per comparison
    println!("{users:?}");
}
