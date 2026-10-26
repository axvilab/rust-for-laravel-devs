//! `&&f64` у `filter`: дві зірочки, деструктуризація або `.copied()`

fn main() {
    let prices = [100.0, 250.0, 80.0];

    let a: Vec<&f64> = prices.iter().filter(|p| **p > 90.0).collect(); // дві зірочки
    let b: Vec<&f64> = prices.iter().filter(|&&p| p > 90.0).collect(); // деструктуризація
    let c: Vec<f64> = prices.iter().copied().filter(|p| *p > 90.0).collect(); // copied()

    println!("{a:?} {b:?} {c:?}");
}
