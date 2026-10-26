//! Several bounds with `+` and a `where` block

fn report<T>(items: &[T]) -> String
where
    T: std::fmt::Display + Clone,
{
    items
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<_>>()
        .join(", ")
}

// the same in angle brackets
fn report_inline<T: std::fmt::Display + Clone>(items: &[T]) -> String {
    report(items)
}

fn main() {
    println!("{}", report(&[1, 2, 3]));
    println!("{}", report_inline(&["a", "b"]));
}
