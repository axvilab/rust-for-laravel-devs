//! Exercise 2: a discount closure applied with `.map()`

fn main() {
    let percent = 0.15;
    let discount = |price: f64| price * (1.0 - percent); // percent is captured by borrowing

    // An array, not vec!: the list never changes (clippy::useless_vec).
    let prices = [100.0, 250.0, 80.0];
    // We pass the closure to map by name — it already takes an f64.
    // iter() yields &f64, so `copied()` turns them into f64.
    let discounted: Vec<f64> = prices.iter().copied().map(discount).collect();
    println!("{discounted:?}"); // [85.0, 212.5, 68.0]
}
