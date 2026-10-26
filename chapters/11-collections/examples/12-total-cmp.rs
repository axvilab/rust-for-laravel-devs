//! `total_cmp` — сортування f64 без паніки на NaN

fn main() {
    let mut prices = vec![2.5, f64::NAN, 1.0];
    prices.sort_by(|a, b| a.total_cmp(b)); // [1.0, 2.5, NaN], без паніки
    println!("{prices:?}");
    // prices.sort_by(|a, b| a.partial_cmp(b).unwrap()); // паніка: unwrap на None через NaN
}
