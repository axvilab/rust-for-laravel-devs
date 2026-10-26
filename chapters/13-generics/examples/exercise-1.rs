//! Exercise 1: `count_where` with a generic `T` and a predicate closure

fn count_where<T>(items: &[T], pred: impl Fn(&T) -> bool) -> usize {
    items.iter().filter(|x| pred(x)).count()
}

fn main() {
    println!("{}", count_where(&[1, 5, 10, 20], |n| *n > 4)); // 3
    println!(
        "{}",
        count_where(&["cat", "car", "dog"], |s| s.starts_with('c'))
    ); // 2
}
