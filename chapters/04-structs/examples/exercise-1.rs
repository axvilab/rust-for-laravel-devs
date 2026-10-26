//! Exercise 1: a `Product` struct with `#[derive(Debug)]`

#[derive(Debug)]
struct Product {
    id: u64,
    name: String,
    price: f64,
    stock: u32,
}

fn main() {
    let p = Product {
        id: 1,
        name: "Laptop".into(),
        price: 45_000.0,
        stock: 3,
    };
    println!("{p:?}");
}
