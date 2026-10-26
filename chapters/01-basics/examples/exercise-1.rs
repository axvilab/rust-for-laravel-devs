//! Exercise 1: price (f64) × quantity (u32) — needs an explicit conversion

fn main() {
    let mut price: f64 = 199.99;
    let qty: u32 = 3;

    // `price * qty` won't compile: f64 and u32 are different types.
    // Convert the quantity explicitly with `as f64`.
    let total = price * qty as f64;
    println!("total: {total}");

    // `mut` was needed because we change the price (a discount):
    price -= 10.0;
    println!("new price: {price}");
}
