//! Sorting a `Vec`: sort, sort_by, sort_by_key, partial_cmp for f64

#[derive(Debug)]
struct User {
    name: String,
    age: u32,
}

fn main() {
    let mut nums = vec![3, 1, 2];
    nums.sort(); // 1 2 3
    println!("{nums:?}");
    nums.sort_by(|a, b| b.cmp(a)); // 3 2 1: cmp returns Less/Equal/Greater
    println!("{nums:?}");

    let mut users = vec![
        User {
            name: "Oleh".into(),
            age: 41,
        },
        User {
            name: "Ivan".into(),
            age: 29,
        },
        User {
            name: "Maria".into(),
            age: 35,
        },
    ];
    users.sort_by_key(|u| u.age); // by a field
    println!("{users:?}");
    users.sort_by(|a, b| a.name.cmp(&b.name)); // by a string
    println!("{users:?}");

    let mut prices = vec![2.5, 1.0];
    prices.sort_by(|a, b| a.partial_cmp(b).unwrap()); // f64 isn't Ord because of NaN
    println!("{prices:?}");
}
