//! The bound `T: PartialOrd` — "any type that can be compared"

fn largest<T: PartialOrd>(items: &[T]) -> &T {
    let mut max = &items[0];
    for item in items {
        if item > max {
            max = item;
        }
    }
    max
}

fn main() {
    println!("{}", largest(&[3, 7, 2])); // 7
    println!("{}", largest(&[1.5, -2.0])); // 1.5
    println!("{}", largest(&["cat", "dog", "ant"])); // dog (alphabetically)
}
