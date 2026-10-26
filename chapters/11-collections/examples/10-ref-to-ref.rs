//! `&&f64` in `filter`: two asterisks, destructuring or `.copied()`

fn main() {
    let prices = [100.0, 250.0, 80.0];

    let a: Vec<&f64> = prices.iter().filter(|p| **p > 90.0).collect(); // two asterisks
    let b: Vec<&f64> = prices.iter().filter(|&&p| p > 90.0).collect(); // destructuring
    let c: Vec<f64> = prices.iter().copied().filter(|p| *p > 90.0).collect(); // copied()

    println!("{a:?} {b:?} {c:?}");
}
