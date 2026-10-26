//! Вправа 2: два знаки після коми через `{:.2}`

fn main() {
    let price: f64 = 199.99;
    let qty: u32 = 3;
    let total = price * qty as f64;
    println!("сума: {total:.2}"); // 599.97 — без «хвоста» 599.9700000000001
}
