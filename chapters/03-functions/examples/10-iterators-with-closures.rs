//! `.map()`, `.filter()`, `.sum()` — як методи Laravel-колекції

fn main() {
    // У книзі `vec![...]`; clippy (useless_vec) радить масив, бо список не змінюється.
    let prices = [100.0, 250.0, 80.0];

    let with_tax: Vec<f64> = prices.iter().map(|p| p * 1.2).collect();
    let expensive: Vec<&f64> = prices.iter().filter(|p| **p > 90.0).collect();
    let total: f64 = prices.iter().sum();

    println!("{with_tax:?}"); // [120.0, 300.0, 96.0]
    println!("{expensive:?}"); // [100.0, 250.0]
    println!("{total}"); // 430
}
