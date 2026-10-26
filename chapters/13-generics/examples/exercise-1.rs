//! Вправа 1: `count_where` з generic `T` і замиканням-предикатом

fn count_where<T>(items: &[T], pred: impl Fn(&T) -> bool) -> usize {
    items.iter().filter(|x| pred(x)).count()
}

fn main() {
    println!("{}", count_where(&[1, 5, 10, 20], |n| *n > 4)); // 3
    println!(
        "{}",
        count_where(&["кіт", "кит", "пес"], |s| s.starts_with('к'))
    ); // 2
}
