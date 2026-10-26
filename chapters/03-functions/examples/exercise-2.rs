//! Вправа 2: замикання зі знижкою, застосоване через `.map()`

fn main() {
    let percent = 0.15;
    let discount = |price: f64| price * (1.0 - percent); // percent захоплено позичанням

    // У книзі `vec![...]`; clippy (useless_vec) радить масив, бо список не змінюється.
    let prices = [100.0, 250.0, 80.0];
    // Замикання передаємо в map за іменем — воно вже приймає f64.
    // iter() дає &f64, тому `copied()` перетворює їх на f64.
    let discounted: Vec<f64> = prices.iter().copied().map(discount).collect();
    println!("{discounted:?}"); // [85.0, 212.5, 68.0]
}
