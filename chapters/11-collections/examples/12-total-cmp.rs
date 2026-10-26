//! `total_cmp` — sorting f64 without panicking on NaN

fn main() {
    let mut prices = vec![2.5, f64::NAN, 1.0];
    prices.sort_by(|a, b| a.total_cmp(b)); // [1.0, 2.5, NaN], no panic
    println!("{prices:?}");
    // prices.sort_by(|a, b| a.partial_cmp(b).unwrap()); // panic: unwrap on None because of NaN
}
