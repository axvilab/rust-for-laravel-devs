//! Exercise 2: two decimal places with `{:.2}`

fn main() {
    let price: f64 = 199.99;
    let qty: u32 = 3;
    let total = price * qty as f64;
    println!("total: {total:.2}"); // 599.97 — without the 599.9700000000001 "tail"
}
